use crate::meeting::Position;
use chrono::Utc;
use rusqlite::{params, Connection, OpenFlags};
use serde::Deserialize;
use std::path::Path;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::sleep;

const API_RATE_LIMIT: Duration = Duration::from_secs(2);

#[derive(Debug, Error)]
pub enum PositionLookupError {
    #[error("SQL error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("HTTP Request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct PositionLookupValue {
    pub position: Option<Position>,
    pub cached: bool,
}

pub struct PositionLookup {
    last_api_request: Mutex<SystemTime>,
    cache_conn: Connection,
}

impl PositionLookup {
    pub fn open(path: &Path) -> Result<Self, PositionLookupError> {
        let mut conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )?;

        Self::migrate(&mut conn)?;

        Ok(Self {
            cache_conn: conn,
            last_api_request: Mutex::new(SystemTime::now()),
        })
    }

    fn migrate(conn: &mut Connection) -> Result<(), PositionLookupError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS positions (
            query TEXT NOT NULL PRIMARY KEY,
            latitude REAL NULL,
            longitude REAL NULL,
            requested_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
            params![],
        )?;

        Ok(())
    }

    pub async fn search(&self, query: &str) -> Result<PositionLookupValue, PositionLookupError> {
        let cached = self.get_cached_position(query)?;

        if let Some(position) = cached {
            return Ok(position);
        }

        let position = self.get_position_from_api(query).await?;

        self.set_cached_position(query, &position)?;

        Ok(PositionLookupValue {
            position,
            cached: false,
        })
    }

    fn get_cached_position(
        &self,
        query: &str,
    ) -> Result<Option<PositionLookupValue>, PositionLookupError> {
        // Invalidate after 1 month
        let mut stmt = self.cache_conn.prepare(
            "
SELECT * FROM positions
WHERE `query` = ? AND requested_at > date('now', '-1 month')",
        )?;

        let mut rows = stmt.query_map(params![query], |row| {
            let position = match (row.get("latitude")?, row.get("longitude")?) {
                (Some(latitude), Some(longitude)) => Some(Position::new(latitude, longitude)),
                _ => None,
            };

            Ok(PositionLookupValue {
                position,
                cached: true,
            })
        })?;

        if let Some(row) = rows.next() {
            Ok(Some(row?))
        } else {
            Ok(None)
        }
    }

    fn set_cached_position(
        &self,
        query: &str,
        position: &Option<Position>,
    ) -> Result<(), PositionLookupError> {
        self.cache_conn.execute("INSERT OR REPLACE INTO positions (`query`, latitude, longitude, requested_at) values(?, ?, ?, ?)", params![
            query,
            position.as_ref().map(|p| p.latitude),
            position.as_ref().map(|p| p.longitude),
            Utc::now()
        ])?;
        Ok(())
    }

    async fn get_position_from_api(
        &self,
        query: &str,
    ) -> Result<Option<Position>, PositionLookupError> {
        let mut lock = self.api_lock_and_ratelimit().await;

        let res: ApiData = reqwest::get(format!(
            "https://positionstack.com/geo_api.php?query={query}"
        ))
        .await?
        .json()
        .await?;

        *lock = SystemTime::now();

        if let Some(record) = res.data.into_iter().next() {
            Ok(Some(record.into()))
        } else {
            Ok(None)
        }
    }

    async fn api_lock_and_ratelimit(&self) -> MutexGuard<'_, SystemTime> {
        let lock = self.last_api_request.lock().await;
        let now = SystemTime::now();

        let elapsed = now.duration_since(lock.clone()).unwrap();

        if let Some(sleep_time) = API_RATE_LIMIT.checked_sub(elapsed) {
            sleep(sleep_time).await;
        }

        lock
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiData {
    pub data: Vec<ApiRecord>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiRecord {
    pub latitude: f64,
    pub longitude: f64,
}

impl Into<Position> for ApiRecord {
    fn into(self) -> Position {
        Position::new(self.latitude, self.longitude)
    }
}
