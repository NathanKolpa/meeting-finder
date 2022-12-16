use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use r2d2::Pool;
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

pub struct MeetingImport<'index> {
    tx: Transaction<'index>,
}

impl<'index> MeetingImport<'index> {
    pub async fn add_meetings(&self, meetings: Vec<Meeting>) -> Result<(), IndexError> {
        Ok(())
    }

    pub async fn commit(self) -> Result<(), IndexError> {
        self.tx.commit()?;
        Ok(())
    }
}

pub struct MeetingIndex {
    conn: Connection,
}

impl MeetingIndex {
    pub fn open(path: &Path) -> Result<Self, IndexError> {
        let mut conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )?;

        Self::migrate(&mut conn)?;

        Ok(Self { conn })
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
            staging INTEGER NOT NULL
        )",
            params![],
        )?;

        Ok(())
    }

    pub async fn start_import(&mut self) -> Result<MeetingImport<'_>, IndexError> {
        Ok(MeetingImport {
            tx: self.conn.transaction()?,
        })
    }
}
