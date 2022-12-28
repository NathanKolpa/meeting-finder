use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub position: Option<Position>,
    pub name: Option<String>,
    pub notes: Option<String>,
    pub country: Option<String>,
    pub region: Option<String>,
    pub address: Option<String>,
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[derive(Debug, Clone, PartialEq)]
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

impl ToString for WeekDay {
    fn to_string(&self) -> String {
        match self {
            WeekDay::Monday => String::from("Monday"),
            WeekDay::Tuesday => String::from("Tuesday"),
            WeekDay::Wednesday => String::from("Wednesday"),
            WeekDay::Thursday => String::from("Thursday"),
            WeekDay::Friday => String::from("Friday"),
            WeekDay::Saturday => String::from("Saturday"),
            WeekDay::Sunday => String::from("Sunday"),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[derive(Debug, Clone, PartialEq)]
pub enum MeetingTime {
    #[serde(rename = "recurring")]
    Recurring {
        day: WeekDay,
        hour: i32,
        minute: i32,
    },
}

impl ToString for MeetingTime {
    fn to_string(&self) -> String {
        match self {// TODO: use display
            MeetingTime::Recurring { hour, minute, day } => {
                format!("Every {} at {:02}:{:02}", day.to_string(), hour, minute)
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Organization {
    AnonymousAlcoholics,
    DebtorsAnonymous,
    CrystalMethAnonymous,
    CodependentsAnonymous,
    NarcoticsAnonymous,
}

impl ToString for Organization {
    fn to_string(&self) -> String {
        match self {
            Organization::AnonymousAlcoholics => String::from("AnonymousAlcoholics"),
            Organization::DebtorsAnonymous => String::from("DebtorsAnonymous"),
            Organization::CrystalMethAnonymous => String::from("CrystalMethAnonymous"),
            Organization::CodependentsAnonymous => String::from("CodependentsAnonymous"),
            Organization::NarcoticsAnonymous => String::from("NarcoticsAnonymous"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
            "NarcoticsAnonymous" => Ok(Self::NarcoticsAnonymous),
            _ => Err(OrganizationParseError::UnknownOrg),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[derive(Debug, Clone, PartialEq)]
pub struct OnlineOptions {
    pub url: Option<String>,
    pub notes: Option<String>,
    pub is_online: bool,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[derive(Debug, Clone, PartialEq)]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
#[derive(Debug, Clone, PartialEq)]
pub struct SearchMeeting {
    pub meeting: Meeting,
    pub distance: Option<f64>,
}