use chrono::{Duration, NaiveTime};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

impl Position {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub position: Option<Position>,
    pub location_name: Option<String>,
    pub location_notes: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MeetingTime {
    Recurring { day: u8, time: NaiveTime },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Organization {
    AnonymousAlcoholics,
}

#[derive(Debug, Clone)]
pub struct Meeting {
    pub contact: Contact,
    pub location: Location,
    pub confrence_url: Option<String>,

    pub time: MeetingTime,
    pub duration: Duration,

    pub notes: Option<String>,
    pub org: Organization,
}
