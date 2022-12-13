use chrono::{DateTime, Duration, Utc};

pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

pub struct Location {
    pub position: Option<Position>,
}

pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
}

pub struct Meeting {
    pub contact: Contact,
    pub location: Location,
    pub confrence_url: Option<String>,

    pub time: DateTime<Utc>,
    pub duration: Duration,

    pub notes: Option<String>,
}
