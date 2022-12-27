use std::time::Duration;

use chrono::{NaiveTime, Timelike, Utc};
use futures_util::future::join_all;
use serde::Deserialize;
use tokio::sync::mpsc::Sender;

use crate::meeting::*;
use crate::source::{FetchMeeting, FetchMeetingResult, MeetingFetchError};

async fn fetch_all_meetings(api_url: &str) -> FetchMeetingResult {
    let query = "switcher=GetSearchResults&get_used_formats&lang_enum=en&data_field_key=location_postal_code_1,duration_time,start_time,time_zone,weekday_tinyint,service_body_bigint,location_province,location_municipality,location_street,location_info,location_neighborhood,formats,comments,location_sub_province,worldid_mixed,root_server_uri,id_bigint,venue_type,meeting_name,location_text,virtual_meeting_link,phone_meeting_number,latitude,longitude,contact_name_1,contact_phone_1,contact_email_1,contact_name_2,contact_phone_2,contact_email_2&callback=callback";
    let url = format!("{api_url}?{query}");

    let response_text = reqwest::get(&url).await?.text().await?;

    let json = &response_text["callback(".len()..response_text.len() - ");".len()];

    let data: ApiData = serde_json::from_str(json)?;

    Ok(data
        .meetings
        .into_iter()
        .filter_map(|m| m.try_into().ok())
        .collect())
}

async fn fetch_and_send(api_url: String, output: Sender<FetchMeetingResult>) {
    let result = fetch_all_meetings(&api_url).await;
    output.send(result).await.unwrap();
}

async fn fetch_from_all_servers(
    output: Sender<FetchMeetingResult>,
) -> Result<(), MeetingFetchError> {
    let servers: Vec<BmltServer> =
        reqwest::get("https://tomato.bmltenabled.org/main_server/api/v1/rootservers/")
            .await?
            .json()
            .await?;

    let futures = servers.iter().map(|server| {
        fetch_and_send(
            format!("{}client_interface/jsonp/", server.url),
            output.clone(),
        )
    });

    join_all(futures).await;

    Ok(())
}

pub async fn fetch_meetings(output: Sender<FetchMeetingResult>) {
    if let Err(e) = fetch_from_all_servers(output.clone()).await {
        output.send(Err(e)).await.unwrap();
    }
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BmltServer {
    pub id: i64,
    pub source_id: i64,
    pub name: String,
    pub url: String,
    pub server_info: String,
    pub last_successful_import: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiData {
    pub meetings: Vec<ApiMeeting>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiMeeting {
    #[serde(rename = "id_bigint")]
    pub id_bigint: String,
    #[serde(rename = "weekday_tinyint")]
    pub weekday_tinyint: String,
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "duration_time")]
    pub duration_time: String,
    pub formats: String,
    pub longitude: String,
    pub latitude: String,
    #[serde(rename = "meeting_name")]
    pub meeting_name: String,
    #[serde(rename = "location_text")]
    pub location_text: String,
    #[serde(rename = "location_info")]
    pub location_info: String,
    #[serde(rename = "location_street")]
    pub location_street: String,
    #[serde(rename = "location_neighborhood")]
    pub location_neighborhood: String,
    #[serde(rename = "location_municipality")]
    pub location_municipality: String,
    #[serde(rename = "location_sub_province")]
    pub location_sub_province: String,
    #[serde(rename = "location_province")]
    pub location_province: String,
    #[serde(rename = "location_postal_code_1")]
    pub location_postal_code_1: String,
    pub comments: String,
    #[serde(rename = "contact_phone_2")]
    pub contact_phone_2: String,
    #[serde(rename = "contact_email_2")]
    pub contact_email_2: String,
    #[serde(rename = "contact_name_2")]
    pub contact_name_2: String,
    #[serde(rename = "contact_phone_1")]
    pub contact_phone_1: String,
    #[serde(rename = "contact_email_1")]
    pub contact_email_1: String,
    #[serde(rename = "contact_name_1")]
    pub contact_name_1: String,
    #[serde(rename = "phone_meeting_number")]
    pub phone_meeting_number: String,
    #[serde(rename = "virtual_meeting_link")]
    pub virtual_meeting_link: String,
    #[serde(rename = "root_server_uri")]
    pub root_server_uri: String,
}

impl TryInto<FetchMeeting> for ApiMeeting {
    type Error = ();

    fn try_into(self) -> Result<FetchMeeting, Self::Error> {
        let start_time = NaiveTime::parse_from_str(&self.start_time, "%H:%M:%S").map_err(|_| ())?;
        let duration =
            NaiveTime::parse_from_str(&self.duration_time, "%H:%M:%S").map_err(|_| ())?;

        let email = if !self.contact_email_1.is_empty() {
            Some(self.contact_email_1)
        } else if !self.contact_email_2.is_empty() {
            Some(self.contact_email_2)
        } else {
            None
        };

        let phone = if !self.contact_phone_1.is_empty() {
            Some(self.contact_phone_1)
        } else if !self.contact_phone_2.is_empty() {
            Some(self.contact_phone_2)
        } else {
            None
        };

        let is_online = self.formats.contains("VM");

        Ok(FetchMeeting {
            position_query: None,
            meeting: Meeting {
                name: self.meeting_name,
                org: Organization::NarcoticsAnonymous,
                notes: if self.comments.is_empty() {
                    None
                } else {
                    Some(self.comments)
                },
                source: self.root_server_uri.trim_end_matches("/").trim_end_matches("/main_server").to_string(),
                updated_at: Utc::now(),
                contact: Contact { email, phone },
                location: Location {
                    position: Some(Position::new(
                        self.latitude.parse().map_err(|_| ())?,
                        self.longitude.parse().map_err(|_| ())?,
                    )),
                    name: if self.location_text.is_empty() {
                        None
                    } else {
                        Some(self.location_text)
                    },
                    notes: if self.location_info.is_empty() {
                        None
                    } else {
                        Some(self.location_info)
                    },
                    country: Some(String::from("United States")),
                    region: Some(self.location_province),
                    address: if !self.location_street.is_empty() {
                        Some(self.location_street)
                    } else {
                        None
                    },
                },
                online_options: OnlineOptions {
                    url: if self.virtual_meeting_link.is_empty() || !is_online {
                        None
                    } else {
                        Some(self.virtual_meeting_link)
                    },
                    notes: if self.phone_meeting_number.is_empty() {
                        None
                    } else {
                        Some(self.phone_meeting_number)
                    },
                    is_online,
                },
                time: MeetingTime::Recurring {
                    day: WeekDay::from_day_index(
                        self.weekday_tinyint.parse::<u8>().map_err(|_| ())? - 1,
                    ),
                    minute: start_time.minute() as i32,
                    hour: start_time.hour() as i32,
                },
                duration: Some(Duration::from_secs(
                    (duration.minute() * 60 + (duration.hour() * 60 * 60) + duration.second())
                        as u64,
                )),
            },
        })
    }
}
