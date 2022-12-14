use chrono::{DateTime, Duration, NaiveTime, Utc};

#[derive(Debug)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub struct Location {
    pub position: Option<Position>,
    pub location_name: Option<String>,
    pub location_notes: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug)]
pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug)]
pub enum MeetingTime {
    Recurring { day: u8, time: NaiveTime },
}

#[derive(Debug)]
pub struct Meeting {
    pub contact: Contact,
    pub location: Location,
    pub confrence_url: Option<String>,

    pub time: MeetingTime,
    pub duration: Duration,

    pub notes: Option<String>,
}
