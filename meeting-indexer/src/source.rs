mod aa;

use async_trait::async_trait;
use tokio::{join, sync::mpsc::Sender};

use crate::meeting::Meeting;

pub enum MeetingFetchError {}

pub type FetchMeetingResult = Result<Vec<Meeting>, MeetingFetchError>;

#[async_trait]
trait MeetingSource {
    async fn fetch_meetings(output: Sender<FetchMeetingResult>);
}

pub async fn fetch_all_meetings(output: Sender<FetchMeetingResult>) {
    join!(aa::AA::fetch_meetings(output.clone()));
}
