use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use rusqlite::{params, Connection, OpenFlags, Transaction};
use thiserror::Error;
use tokio::task::spawn_blocking;

use crate::meeting::{Meeting, MeetingTime};

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
                "INSERT INTO meetings(latitude, longitude, location_name, location_notes, country, region, address, notes, org, confrence_url, phone, email, duration, day, time)
                VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    meeting.location.position.as_ref().map(|p| p.latitude),
                    meeting.location.position.as_ref().map(|p| p.longitude),
                    meeting.location.location_name,
                    meeting.location.location_notes,
                    meeting.location.country,
                    meeting.location.region,
                    meeting.location.address,
                    meeting.notes,
                    meeting.org.to_string(),
                    meeting.confrence_url,
                    meeting.contact.phone,
                    meeting.contact.email,
                    meeting.duration.to_string(),
                    meeting_day,
                    meeting_time
                ])?;
        }

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
