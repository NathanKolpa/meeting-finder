use tokio::sync::mpsc::Sender;

use super::{FetchMeetingResult, MeetingSource};
use async_trait::async_trait;

pub struct AA;

#[async_trait]
impl MeetingSource for AA {
    async fn fetch_meetings(output: Sender<FetchMeetingResult>) {}
}
