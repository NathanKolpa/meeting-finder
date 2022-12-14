use std::{
    path::Path,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex, MutexGuard,
    },
};

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection, OpenFlags, Transaction};
use thiserror::Error;
use tokio::task::spawn_blocking;

use crate::meeting::Meeting;

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("SQL error: {0}")]
    SqliteError(#[from] rusqlite::Error),
}

pub struct MeetingIndex {
    write_conn: Arc<Mutex<Connection>>,
    read_pool: Pool<SqliteConnectionManager>,
}

impl MeetingIndex {
    pub fn open(path: &Path) -> Result<Self, IndexError> {
        let mut conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )?;
        Self::migrate(&mut conn)?;

        let manager = r2d2_sqlite::SqliteConnectionManager::file(path)
            .with_flags(OpenFlags::SQLITE_OPEN_READ_ONLY);

        Ok(Self {
            write_conn: Arc::new(Mutex::new(conn)),
            read_pool: Pool::builder().build(manager).unwrap(),
        })
    }

    fn migrate(conn: &mut Connection) -> Result<(), IndexError> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS meetings (
            latitude REAL NULL,
            longitude REAL NULL,
            location_name TEXT NULL,
            location_notes TEXT NULL,
            country TEXT NULL,
            region TEXT NULL,
            address TEXT NULL,


            notes TEXT NULL,
            org TEXT NOT NULL,
            confrence_url TEXT NULL,
            
            phone TEXT NULL,
            email TEXT NULL,

            duration TEXT NOT NULL,
            day INTEGER NULL,
            time TEXT NULL

        )",
            params![],
        )?;

        Ok(())
    }

    pub async fn add_meetings_to_staging(&self, meetings: Vec<Meeting>) -> Result<(), IndexError> {
        Ok(())
    }

    pub async fn commit_staging(&self) -> Result<(), IndexError> {
        let write_conn = self.write_conn.clone();

        spawn_blocking(move || -> Result<(), IndexError> {
            let mut conn = write_conn.lock().unwrap();
            let tx = conn.transaction()?;

            tx.commit()?;
            Ok(())
        })
        .await
        .unwrap()?;

        Ok(())
    }
}
