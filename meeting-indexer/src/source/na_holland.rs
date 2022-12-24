use crate::meeting::*;
use crate::source::{FetchMeeting, FetchMeetingResult};
use chrono::{NaiveTime, Timelike, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

async fn fetch_all_meetings(api_url: &str) -> FetchMeetingResult {
    let data: ApiData = reqwest::get(api_url).await?.json().await?;

    Ok(data
        .meetings
        .into_iter()
        .filter_map(|m| m.try_into().ok())
        .collect())
}

pub async fn fetch_meetings(output: Sender<FetchMeetingResult>) {
    let result = fetch_all_meetings("https://www.na-holland.nl/api/v1/meetings").await;
    output.send(result).await.unwrap();
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiData {
    pub meetings: Vec<ApiRecord>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiRecord {
    pub id: i64,
    #[serde(rename = "province_name")]
    pub province_name: String,
    #[serde(rename = "city_name")]
    pub city_name: String,
    pub weekday: u8,
    pub start: String,
    pub finish: String,
    // Why they put random shit in the address field????
    pub address: String,
    pub contact: String,
    pub details: String,
}

enum ConvertError {
    TimeParseError,
}

impl TryInto<FetchMeeting> for ApiRecord {
    type Error = ConvertError;

    fn try_into(self) -> Result<FetchMeeting, Self::Error> {
        lazy_static! {
            static ref STREET_REGEX: Regex = Regex::new(r"[a-zA-Z]+[ ]\d+[a-zA-Z]*,").unwrap();
        }

        let start_time = NaiveTime::parse_from_str(&self.start, "%H:%M")
            .map_err(|_| ConvertError::TimeParseError)?;

        let end_time = NaiveTime::parse_from_str(&self.finish, "%H:%M")
            .map_err(|_| ConvertError::TimeParseError)?;

        let query = if let Some(street_match) = STREET_REGEX.find(&self.address) {
            let mut value = street_match.as_str().to_string();
            value.pop(); // remove ","
            Some(format!("{value} {}", self.city_name))
        } else {
            None
        };

        Ok(FetchMeeting {
            position_query: query,
            meeting: Meeting {
                name: format!("NA Holland | {} {}", self.city_name, self.address),
                org: Organization::NarcoticsAnonymous,
                notes: Some(self.details),
                source: format!("https://www.na-holland.nl/#/meetings/{}", self.id),
                updated_at: Utc::now(),
                contact: Contact {
                    email: None,
                    phone: None,
                },
                online_options: OnlineOptions {
                    url: None,
                    notes: None,
                    is_online: self.province_name == "ONLINE",
                },
                location: Location {
                    position: None,
                    name: None,
                    notes: None,
                    country: Some(String::from("Nederland")),
                    region: Some(self.province_name),
                    address: Some(self.address),
                },
                time: MeetingTime::Recurring {
                    day: WeekDay::from_day_index(self.weekday - 1),
                    hour: start_time.hour() as i32,
                    minute: start_time.minute() as i32,
                },
                duration: (end_time - start_time).to_std().ok(),
            },
        })
    }
}
