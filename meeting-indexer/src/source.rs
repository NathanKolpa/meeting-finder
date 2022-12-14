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

async fn meeting_recv_and_callback<F, Fut>(mut rx: Receiver<FetchMeetingResult>, f: F)
where
    F: Fn(FetchMeetingResult) -> Fut,
    Fut: Future<Output = ()>,
{
    while let Some(result) = rx.recv().await {
        f(result).await;
    }
}

pub async fn fetch_all_meetings_foreach<F, Fut>(f: F)
where
    F: Fn(FetchMeetingResult) -> Fut,
    Fut: Future<Output = ()>,
{
    let (tx, rx) = channel(100);
    join!(fetch_all_meetings(tx), meeting_recv_and_callback(rx, f));
}
