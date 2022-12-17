use std::collections::HashMap;
use std::string::ParseError;
use std::time::Duration;

use chrono::{DateTime, NaiveTime, ParseResult, Utc};
use clap::builder::Str;
use select::document::Document;
use select::predicate::Attr;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::join;
use tokio::sync::mpsc::Sender;

use crate::meeting::*;
use crate::source::MeetingFetchError;

use super::FetchMeetingResult;

struct Metadata {
    nonce: String,
    meeting_type_map: HashMap<String, String>,
    endpoint: String,
}

async fn fetch_metadata(meeting_url: &str) -> Result<Metadata, MeetingFetchError> {
    let page_text = reqwest::get(meeting_url)
        .await?
        .text()
        .await?;

    let document = Document::from(page_text.as_str());

    let script_element = document.find(Attr("id", "tsml_public-js-extra"))
        .next()
        .ok_or_else(|| MeetingFetchError::UnexpectedResponse(String::from("Cannot find tsml metadata script")))?;

    let mut text = script_element.text();

    let json_start_index = text.find('{').ok_or_else(|| MeetingFetchError::UnexpectedResponse(String::from("cannot find the start of the json text")))?;
    let json_end_index = text.rfind('}').ok_or_else(|| MeetingFetchError::UnexpectedResponse(String::from("cannot find the end of the json text")))?;
    let json_text = &text[json_start_index..json_end_index + 1];

    let json: Value = serde_json::from_str(json_text)?;

    let map = json.get("types")
        .ok_or_else(|| MeetingFetchError::UnexpectedResponse(String::from("Cannot find 'types' in json")))?
        .as_object()
        .ok_or_else(|| MeetingFetchError::UnexpectedResponse(String::from("'types' is not a json object")))?;

    let mut meeting_type_map = HashMap::new();

    for (key, value) in map.iter() {
        let value = value.as_str()
            .ok_or_else(|| MeetingFetchError::UnexpectedResponse(String::from("'types.*' is not a string")))?;

        meeting_type_map.insert(key.clone(), value.to_string());
    }

    Ok(Metadata {
        nonce: json.get("nonce")
            .ok_or_else(|| MeetingFetchError::UnexpectedResponse(String::from("Cannot find 'nonce' in json")))?
            .as_str()
            .unwrap()
            .to_string(),

        endpoint: json.get("ajaxurl")
            .ok_or_else(|| MeetingFetchError::UnexpectedResponse(String::from("Cannot find 'ajaxurl' in json")))?
            .as_str()
            .unwrap()
            .to_string(),

        meeting_type_map,
    })
}

async fn fetch_all_meetings(meetings_url: &str, org: Organization) -> FetchMeetingResult {
    let metadata = fetch_metadata(meetings_url).await?;

    let params = [
        ("action", "meetings"),
        ("mode", "search"),
        ("distance", "2"),
        ("view", "list"),
        ("distance_units", "km"),
        ("nonce", &metadata.nonce),
    ];

    let client = reqwest::Client::new();
    let res: Vec<AAMeeting> = client
        .post(&metadata.endpoint)
        .form(&params)
        .send()
        .await?
        .json()
        .await?;

    Ok(res.into_iter().filter_map(|m| {
        m.try_into().ok().map(|mut m: Meeting| {
            m.org = org.clone();
            m
        })
    }).collect())
}

async fn fetch_and_send(org: Organization, meetings_url: &str, output: Sender<FetchMeetingResult>) {
    let result = fetch_all_meetings(meetings_url, org).await;
    output.send(result).await.unwrap();
}

pub async fn fetch_meetings(output: Sender<FetchMeetingResult>) {
    join!(
        fetch_and_send(Organization::AnonymousAlcoholics, "https://alcoholics-anonymous.eu/meetings/?tsml-day=6&tsml-view=map", output.clone()),
        fetch_and_send(Organization::DebtorsAnonymous, "https://debtorsanonymous.org/meetings/?tsml-day=any", output.clone()),
    );
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AAMeeting {
    pub id: i64,
    pub name: String,
    pub slug: Option<String>,
    pub notes: Option<String>,
    pub url: String,
    pub day: Option<Value>,
    #[serde(default)]
    pub types: Vec<String>,
    pub location: Option<String>,
    #[serde(rename = "location_url")]
    pub location_url: Option<String>,
    #[serde(rename = "formatted_address")]
    pub formatted_address: Option<String>,
    pub approximate: Option<String>,
    pub latitude: Value,
    pub longitude: Value,
    pub region: Option<String>,
    #[serde(rename = "sub_region")]
    pub sub_region: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(rename = "last_contact")]
    pub last_contact: Option<String>,
    #[serde(rename = "attendance_option")]
    pub attendance_option: Option<String>,
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
    ParseError,
}

impl TryInto<Meeting> for AAMeeting {
    type Error = ConvertError;

    fn try_into(self) -> Result<Meeting, Self::Error> {
        let time = self.time.ok_or(ConvertError::MissingField)?;
        let end_time = self.end_time.ok_or(ConvertError::MissingField)?;

        let day_value = self.day.ok_or(ConvertError::MissingField)?;
        let mut day: u8 = day_value.as_u64().or_else(|| day_value.as_str().map(|s| s.parse().unwrap()))
            .ok_or(ConvertError::ParseError)? as u8;

        let convert_f64 = |val: Value| -> Result<f64, ConvertError> {
            Ok(val.as_f64().or_else(|| val.as_str().map(|s| s.parse().unwrap()))
                .ok_or(ConvertError::ParseError)?)
        };

        let latitude = convert_f64(self.latitude)?;
        let longitude = convert_f64(self.longitude)?;

        // We specify that monday = 0 and sunday = 6.
        if day == 0 {
            day = 6;
        } else {
            day -= 1;
        }

        // TODO: remove all unwraps

        Ok(Meeting {
            online_options: OnlineOptions {
                is_online: self.region.as_ref().map(|region| region == "--Online--").unwrap_or(false),
                online_url: self.conference_url,
                notes: self.conference_url_notes,
            },
            name: self.name,
            source: self.url,
            org: Organization::AnonymousAlcoholics,
            contact: Contact {
                email: self.email,
                phone: self.phone,
            },
            location: Location {
                position: Some(Position {
                    latitude,
                    longitude,
                }),
                location_name: self.location,
                location_notes: self.location_notes,
                country: self.region,
                region: self.sub_region,
                address: self.formatted_address,
            },
            time: MeetingTime::Recurring {
                day: WeekDay::from_day_index(day),
                time: NaiveTime::parse_from_str(&time, "%H:%M").map_err(|_| ConvertError::ParseError)?,
            },
            notes: self.notes,
            duration: (NaiveTime::parse_from_str(&end_time, "%H:%M").unwrap()
                - NaiveTime::parse_from_str(&time, "%H:%M").unwrap()).to_std().unwrap_or(Duration::from_secs(1)),
        })
    }
}
