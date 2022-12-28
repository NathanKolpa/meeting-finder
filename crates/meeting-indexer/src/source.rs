mod na_holland;
mod wp_sites;
mod bmlt;

use thiserror::Error;
use tokio::{join, sync::mpsc::Sender};

use common::model::Meeting;

#[derive(Error, Debug)]
pub enum MeetingFetchError {
    #[error("HTTP Request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),

    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct FetchMeeting {
    pub meeting: Meeting,
    pub position_query: Option<String>,
}

pub type FetchMeetingResult = Result<Vec<FetchMeeting>, MeetingFetchError>;

pub async fn fetch_all_meetings(output: Sender<FetchMeetingResult>) {
    join!(
        wp_sites::fetch_meetings(output.clone()),
        na_holland::fetch_meetings(output.clone()),
        bmlt::fetch_meetings(output.clone()),
    );
}
