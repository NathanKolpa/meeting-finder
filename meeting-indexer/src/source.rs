mod aa_eu;

use std::future::Future;

use thiserror::Error;
use tokio::{
    join,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::meeting::Meeting;

#[derive(Error, Debug)]
pub enum MeetingFetchError {
    #[error("HTTP Request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),
}

pub type FetchMeetingResult = Result<Vec<Meeting>, MeetingFetchError>;

pub async fn fetch_all_meetings(output: Sender<FetchMeetingResult>) {
    join!(aa_eu::fetch_meetings(output.clone()));
}
