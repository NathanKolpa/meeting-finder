mod aa_eu;

use thiserror::Error;
use tokio::{join, sync::mpsc::Sender};

use crate::meeting::Meeting;

#[derive(Error, Debug)]
pub enum MeetingFetchError {
    #[error("HTTP Request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),

    #[error("JSON parse error: {0}")]
    JsonParseError(#[from] serde_json::Error),
}

pub type FetchMeetingResult = Result<Vec<Meeting>, MeetingFetchError>;

pub async fn fetch_all_meetings(output: Sender<FetchMeetingResult>) {
    join!(aa_eu::fetch_meetings(output.clone()));
}
