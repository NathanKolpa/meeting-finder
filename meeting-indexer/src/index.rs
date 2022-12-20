use std::{
    path::Path,
    sync::{Arc, Mutex},
};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use chrono::NaiveTime;
use rusqlite::{Connection, named_params, OpenFlags, params, ToSql, Transaction};
use serde::Serialize;
use thiserror::Error;
use tokio::task::spawn_blocking;

use crate::meeting::*;

pub struct DistanceSearch {
    pub latitude: f64,
    pub longitude: f64,
    pub distance: f64,
}

#[derive(Default)]
pub struct SearchOptions {
    pub distance: Option<DistanceSearch>,
}

#[derive(Serialize)]
pub struct SearchMeeting {
    pub meeting: Meeting,
    pub distance: Option<f64>,
}

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("SQL error: {0}")]
    SqliteError(#[from] rusqlite::Error),
}

pub struct MeetingImport<'index> {
    tx: Transaction<'index>,
    total_meetings: AtomicUsize,
}

impl<'index> MeetingImport<'index> {
    pub async fn add_meetings(&self, meetings: Vec<Meeting>) -> Result<(), IndexError> {
        let meeting_count = meetings.len();

        for meeting in meetings {
            let mut meeting_day = None;
            let mut meeting_time = None;

            match &meeting.time {
                MeetingTime::Recurring { day, time } => {
                    meeting_day = Some(day);
                    meeting_time = Some(time);
                }
            }

            self.tx.execute(
                "INSERT INTO meetings(online, online_notes, source, latitude, longitude, location_name, location_notes, country, region, address, name, notes, org, online_url, phone, email, duration, day, time)
                VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    meeting.online_options.is_online,
                    meeting.online_options.notes,
                    meeting.source,
                    meeting.location.position.as_ref().map(|p| p.latitude),
                    meeting.location.position.as_ref().map(|p| p.longitude),
                    meeting.location.location_name,
                    meeting.location.location_notes,
                    meeting.location.country,
                    meeting.location.region,
                    meeting.location.address,
                    meeting.name,
                    meeting.notes,
                    meeting.org.to_string(),
                    meeting.online_options.online_url,
                    meeting.contact.phone,
                    meeting.contact.email,
                    meeting.duration.map(|d| d.as_secs()),
                    meeting_day.map(|day| day.to_day_index()),
                    meeting_time
                ])?;
        }

        self.total_meetings.fetch_add(meeting_count, Ordering::Relaxed);

        Ok(())
    }

    pub async fn remove_old_meetings(&self) -> Result<(), IndexError> {
        self.tx.execute("DELETE FROM meetings", params![])?;
        Ok(())
    }

    pub async fn commit(self) -> Result<(), IndexError> {
        self.tx.commit()?;
        Ok(())
    }

    pub fn meetings_added(&self) -> usize {
        self.total_meetings.load(Ordering::Relaxed)
    }
}

pub struct MeetingIndex {
    conn: Connection,
    path: PathBuf,
}

impl Clone for MeetingIndex {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            conn: Connection::open_with_flags(&self.path, OpenFlags::SQLITE_OPEN_READ_WRITE)
                .unwrap(),
        }
    }
}

impl MeetingIndex {
    pub fn open(path: &Path) -> Result<Self, IndexError> {
        let mut conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )?;

        Self::migrate(&mut conn)?;

        Ok(Self { conn, path: path.to_path_buf() })
    }

    fn migrate(conn: &mut Connection) -> Result<(), IndexError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS meetings (
            name TEXT NOT NULL,
            org TEXT NOT NULL,
            notes TEXT NULL,
            source TEXT NOT NULL,

            country TEXT NULL,
            region TEXT NULL,
            location_name TEXT NULL,
            location_notes TEXT NULL,
            latitude REAL NULL,
            longitude REAL NULL,
            address TEXT NULL,

            online INTEGER NOT NULL,
            online_url TEXT NULL,
            online_notes TEXT NULL,

            phone TEXT NULL,
            email TEXT NULL,

            duration INTEGER NULL,
            day INTEGER NULL,
            time TEXT NULL
        )",
            params![],
        )?;

        Ok(())
    }

    pub async fn search(&self, opts: &SearchOptions) -> Result<Vec<SearchMeeting>, IndexError> {
        let mut query = String::from("SELECT ");

        let mut params: Vec<(&str, &dyn ToSql)> = Vec::new();

        if opts.distance.is_some() {
            // from https://stackoverflow.com/questions/27928/calculate-distance-between-two-latitude-longitude-points-haversine-formula
            query.push_str("(
    12742 * ASIN(SQRT(0.5 - COS((latitude - :lat) * 0.017453292519943295) / 2.0
    +  COS(:lat * 0.017453292519943295) * COS(latitude * 0.017453292519943295)
    * (1.0 - COS((longitude - :long) * 0.017453292519943295)) / 2.0))
) as distance, ")
        }
        else {
            query.push_str("NULL as distance, ");
        }

        query.push_str("* FROM MEETINGS");

        if opts.distance.is_some() {
            query.push_str("\nWHERE distance < :distance\nORDER BY distance")
        }

        if let Some(distance) = &opts.distance {
            params.push((":lat", &distance.latitude));
            params.push((":long", &distance.longitude));
            params.push((":distance", &distance.distance));
        }

        let mut stmt = self.conn.prepare(query.as_str())?;

        let rows = stmt.query_map(params.as_slice(), |row| {
            let position = match (row.get("latitude")?, row.get("longitude")?) {
                (Some(latitude), Some(longitude)) => {
                    Some(Position {
                        latitude,
                        longitude,
                    })
                }
                _ => None
            };

            // TODO: handle parse errors

            Ok(SearchMeeting {
                distance: row.get("distance")?,
                meeting: Meeting {
                    name: row.get("name")?,
                    org: row.get::<_, String>("org")?.parse().unwrap(),
                    notes: row.get("notes")?,
                    source: row.get("source")?,
                    contact: Contact {
                        email: row.get("email")?,
                        phone: row.get("phone")?,
                    },
                    location: Location {
                        position,
                        location_name: row.get("location_name")?,
                        location_notes: row.get("location_notes")?,
                        country: row.get("country")?,
                        region: row.get("region")?,
                        address: row.get("address")?,
                    },
                    online_options: OnlineOptions {
                        online_url: row.get("online_url")?,
                        notes: row.get("online_notes")?,
                        is_online: row.get("online")?,
                    },
                    time: MeetingTime::Recurring {
                        day: WeekDay::from_day_index(row.get("day")?),
                        time: row.get::<_, String>("time")?.parse().unwrap(),
                    },
                    duration: row.get::<_, Option<u64>>("duration")?.map(|secs| Duration::from_secs(secs)),
                }
            })
        })?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub async fn start_import(&mut self) -> Result<MeetingImport<'_>, IndexError> {
        Ok(MeetingImport {
            tx: self.conn.transaction()?,
            total_meetings: Default::default(),
        })
    }
}
