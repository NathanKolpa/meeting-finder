use std::string::ParseError;

use chrono::{DateTime, NaiveTime, ParseResult, Utc};
use serde::{Deserialize, Serialize};

use tokio::sync::mpsc::Sender;

use crate::meeting::*;

use super::FetchMeetingResult;

async fn fetch_all_meetings() -> FetchMeetingResult {
    let client = reqwest::Client::new();

    let params = [
        ("action", "meetings"),
        ("mode", "search"),
        ("distance", "2"),
        ("view", "list"),
        ("distance_units", "km"),
        ("nonce", "969e2c8494"),
    ];

    let res: Vec<AAMeeting> = client
        .post("https://alcoholics-anonymous.eu/wp-admin/admin-ajax.php")
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(res.into_iter().filter_map(|m| m.try_into().ok()).collect())
}

pub async fn fetch_meetings(output: Sender<FetchMeetingResult>) {
    let result = fetch_all_meetings().await;
    output.send(result).await.unwrap();
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AAMeeting {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub notes: Option<String>,
    pub updated: String,
    #[serde(rename = "location_id")]
    pub location_id: i64,
    pub url: String,
    pub day: Option<u8>,
    #[serde(rename = "edit_url")]
    pub edit_url: String,
    #[serde(default)]
    pub types: Vec<String>,
    pub location: Option<String>,
    #[serde(rename = "location_url")]
    pub location_url: String,
    #[serde(rename = "formatted_address")]
    pub formatted_address: String,
    pub approximate: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "region_id")]
    pub region_id: Option<i64>,
    pub region: Option<String>,
    #[serde(rename = "sub_region")]
    pub sub_region: Option<String>,
    pub regions: Vec<Option<String>>,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "last_contact")]
    pub last_contact: Option<String>,
    #[serde(rename = "attendance_option")]
    pub attendance_option: String,
    pub time: Option<String>,
    #[serde(rename = "end_time")]
    pub end_time: Option<String>,
    #[serde(rename = "time_formatted")]
    pub time_formatted: Option<String>,
    #[serde(rename = "conference_url")]
    pub conference_url: Option<String>,
    pub website: Option<String>,
    #[serde(rename = "conference_url_notes")]
    pub conference_url_notes: Option<String>,
    #[serde(rename = "location_notes")]
    pub location_notes: Option<String>,
    #[serde(rename = "group_id")]
    pub group_id: Option<i64>,
    pub group: Option<String>,
    #[serde(rename = "contact_1_name")]
    pub contact_1_name: Option<String>,
    #[serde(rename = "contact_1_email")]
    pub contact_1_email: Option<String>,
    #[serde(rename = "contact_1_phone")]
    pub contact_1_phone: Option<String>,
    #[serde(rename = "contact_2_name")]
    pub contact_2_name: Option<String>,
    #[serde(rename = "contact_2_email")]
    pub contact_2_email: Option<String>,
    #[serde(rename = "contact_2_phone")]
    pub contact_2_phone: Option<String>,
    pub paypal: Option<String>,
    #[serde(rename = "website_2")]
    pub website_2: Option<String>,
    pub district: Option<String>,
    #[serde(rename = "district_id")]
    pub district_id: Option<i64>,
    #[serde(rename = "contact_3_name")]
    pub contact_3_name: Option<String>,
    #[serde(rename = "contact_3_phone")]
    pub contact_3_phone: Option<String>,
    #[serde(rename = "contact_3_email")]
    pub contact_3_email: Option<String>,
    #[serde(rename = "conference_phone")]
    pub conference_phone: Option<String>,
    #[serde(rename = "mailing_address")]
    pub mailing_address: Option<String>,
    #[serde(rename = "group_notes")]
    pub group_notes: Option<String>,
}

enum ConvertError {
    MissingField,
}

impl TryInto<Meeting> for AAMeeting {
    type Error = ConvertError;

    fn try_into(self) -> Result<Meeting, Self::Error> {
        let time = self.time.ok_or(ConvertError::MissingField)?;
        let end_time = self.end_time.ok_or(ConvertError::MissingField)?;
        let day = self.day.ok_or(ConvertError::MissingField)?;

        Ok(Meeting {
            name: self.name,
            org: Organization::AnonymousAlcoholics,
            contact: Contact {
                email: self.email,
                phone: self.phone,
            },
            location: Location {
                position: Some(Position {
                    latitude: self.latitude,
                    longitude: self.longitude,
                }),
                location_name: self.location,
                location_notes: self.location_notes,
                country: self.region,
                region: self.sub_region,
                address: Some(self.formatted_address),
            },
            confrence_url: self.conference_url,
            time: MeetingTime::Recurring {
                day,
                time: NaiveTime::parse_from_str(&time, "%H:%M").unwrap(),
            },
            notes: self.notes,
            duration: NaiveTime::parse_from_str(&end_time, "%H:%M").unwrap()
                - NaiveTime::parse_from_str(&time, "%H:%M").unwrap(),
        })
    }
}
