extern crate core;

use std::error::Error;
use std::path::PathBuf;

use crate::server::start_server;
use crate::source::FetchMeeting;
use clap::{Parser, Subcommand};
use source::FetchMeetingResult;
use tokio::{
    join,
    sync::mpsc::{channel, Receiver},
};

pub mod index;
pub mod position_lookup;
pub mod server;
pub mod source;

#[derive(Subcommand)]
enum Commands {
    /// Synchronize the database
    Sync,

    /// Launch a webserver
    Serve {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,

        #[arg(short, long, default_value_t = String::from("0.0.0.0"))]
        address: String,
    },
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "DIR", default_value_t = String::from("/usr/share/meeting-indexer"))]
    data_dir: String,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let data_path = PathBuf::from(cli.data_dir);
    let meeting_db_path = data_path.join("meetings.db");

    let mut index = index::MeetingIndex::open(&meeting_db_path)?;

    match cli.command {
        Commands::Sync => {
            let position_db_path = data_path.join("positions.db");
            let position_lookup = position_lookup::PositionLookup::open(&position_db_path)?;

            sync_index(&mut index, &position_lookup).await?;
        }
        Commands::Serve { port, address } => {
            start_server(index, address.parse()?, port).await?;
        }
    }

    Ok(())
}

async fn lookup_meeting_positions(
    meetings: &mut Vec<FetchMeeting>,
    position_lookup: &position_lookup::PositionLookup,
) {
    for meeting in meetings.iter_mut() {
        match (&meeting.meeting.location.position, &meeting.position_query) {
            (None, Some(query)) => {
                let lookup_result = position_lookup.search(query.as_str()).await;

                match lookup_result {
                    Ok(lookup) => {
                        let cache_text = if lookup.cached { " (cached)" } else { "" };
                        let position_text = if let Some(position) = &lookup.position {
                            format!("{}, {}", position.longitude, position.latitude)
                        } else {
                            String::from("NULL")
                        };

                        println!("Mapped \"{query}\" to {position_text}{cache_text}");

                        meeting.meeting.location.position = lookup.position;
                    }
                    Err(e) => {
                        eprintln!("Failed to map \"{query}\": {e}");
                    }
                }
            }
            _ => {}
        }
    }
}

async fn add_meetings_to_index(
    mut rx: Receiver<FetchMeetingResult>,
    import: &mut index::MeetingImport<'_>,
    position_lookup: &position_lookup::PositionLookup,
) {
    while let Some(result) = rx.recv().await {
        match result {
            Err(e) => eprintln!("Failed to fetch meetings: {}", e),
            Ok(mut meetings) => {
                let meeting_count = meetings.len();
                println!("Found {meeting_count} meetings");

                lookup_meeting_positions(&mut meetings, position_lookup).await;
                let result = import
                    .add_meetings(meetings.iter().map(|m| &m.meeting))
                    .await;

                if let Err(e) = result {
                    println!("Failed to add meetings to the staging: {}", e);
                } else {
                    println!("Added {meeting_count} meetings to the staging");
                }
            }
        }
    }
}

async fn sync_index(
    index: &mut index::MeetingIndex,
    position_lookup: &position_lookup::PositionLookup,
) -> Result<(), index::IndexError> {
    let mut import = index.start_import().await?;
    import.remove_old_meetings().await?;

    let (tx, rx) = channel(1024);
    join!(
        source::fetch_all_meetings(tx),
        add_meetings_to_index(rx, &mut import, position_lookup)
    );

    let meeting_count = import.meetings_added();

    if meeting_count > 0 {
        import.commit().await?;
        println!("Committed the staging to the database with {meeting_count} meetings total");
    } else {
        eprintln!("Refusing to commit the staging to the database because it contains 0 meetings");
    }

    Ok(())
}
