use chrono::{DateTime, Utc};
use std::str::FromStr;
use std::time::Duration;

use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default, Serialize, ToSchema)]
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

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Location {
    pub position: Option<Position>,
    pub location_name: Option<String>,
    pub location_notes: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum WeekDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekDay {
    pub fn from_day_index(day: u8) -> Self {
        match day {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Saturday,
            6 => Self::Sunday,
            _ => panic!("Day must be between 0 and 6"),
        }
    }

    pub fn to_day_index(&self) -> u8 {
        match self {
            WeekDay::Monday => 0,
            WeekDay::Tuesday => 1,
            WeekDay::Wednesday => 2,
            WeekDay::Thursday => 3,
            WeekDay::Friday => 4,
            WeekDay::Saturday => 5,
            WeekDay::Sunday => 6,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub enum MeetingTime {
    #[serde(rename = "recurring")]
    Recurring {
        day: WeekDay,
        hour: i32,
        minute: i32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, ToSchema)]
pub enum Organization {
    AnonymousAlcoholics,
    DebtorsAnonymous,
    CrystalMethAnonymous,
    CodependentsAnonymous,
}

impl ToString for Organization {
    fn to_string(&self) -> String {
        match self {
            Organization::AnonymousAlcoholics => String::from("AnonymousAlcoholics"),
            Organization::DebtorsAnonymous => String::from("DebtorsAnonymous"),
            Organization::CrystalMethAnonymous => String::from("CrystalMethAnonymous"),
            Organization::CodependentsAnonymous => String::from("CodependentsAnonymous"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OrganizationParseError {
    UnknownOrg,
}

impl FromStr for Organization {
    type Err = OrganizationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AnonymousAlcoholics" => Ok(Self::AnonymousAlcoholics),
            "DebtorsAnonymous" => Ok(Self::DebtorsAnonymous),
            "CrystalMethAnonymous" => Ok(Self::CrystalMethAnonymous),
            "CodependentsAnonymous" => Ok(Self::CodependentsAnonymous),
            _ => Err(OrganizationParseError::UnknownOrg),
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct OnlineOptions {
    pub online_url: Option<String>,
    pub notes: Option<String>,
    pub is_online: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Meeting {
    pub name: String,
    pub org: Organization,
    pub notes: Option<String>,
    pub source: String,
    pub updated_at: DateTime<Utc>,

    pub contact: Contact,
    pub location: Location,

    pub online_options: OnlineOptions,

    pub time: MeetingTime,

    pub duration: Option<Duration>,
}
