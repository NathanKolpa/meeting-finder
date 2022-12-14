#![feature(async_closure)]

use std::error::Error;

use source::FetchMeetingResult;
use tokio::{
    join,
    sync::mpsc::{channel, Receiver},
    task::spawn_blocking,
};

pub mod index;
pub mod meeting;
pub mod source;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let index = index::MeetingIndex::open(&std::path::PathBuf::from("/tmp/test.db"))?;

    sync_index(&index).await?;

    Ok(())
}

async fn add_meetings_to_index(mut rx: Receiver<FetchMeetingResult>, index: &index::MeetingIndex) {
    while let Some(result) = rx.recv().await {
        match result {
            Err(e) => eprintln!("Failed to fetch meetings: {}", e),
            Ok(meetings) => {
                let _ = index.add_meetings_to_staging(meetings).await;
            }
        }
    }
}

async fn sync_index(index: &index::MeetingIndex) -> Result<(), index::IndexError> {
    let (tx, rx) = channel(100);
    join!(
        source::fetch_all_meetings(tx),
        add_meetings_to_index(rx, &index)
    );

    index.commit_staging().await?;
    Ok(())
}
