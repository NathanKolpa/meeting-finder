#![feature(async_closure)]

extern crate core;

use std::error::Error;

use clap::{Parser, Subcommand};
use source::FetchMeetingResult;
use tokio::{
    join,
    sync::mpsc::{channel, Receiver},
};

pub mod index;
pub mod meeting;
pub mod server;
pub mod source;

#[derive(Subcommand)]
enum Commands {
    /// Syncronize the database
    Sync,

    /// Launch a webserver
    Serve,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The database file used for caching and querying.
    /// A new database file will be created if it doesn't exist.
    #[arg(short, long, value_name = "FILE")]
    db: String,

    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let mut index = index::MeetingIndex::open(&std::path::PathBuf::from(cli.db))?;

    match cli.command {
        Commands::Sync => {
            sync_index(&mut index).await?;
        }
        Commands::Serve => todo!(),
    }

    Ok(())
}

async fn add_meetings_to_index(
    mut rx: Receiver<FetchMeetingResult>,
    import: &mut index::MeetingImport<'_>,
) {
    while let Some(result) = rx.recv().await {
        match result {
            Err(e) => eprintln!("Failed to fetch meetings: {}", e),
            Ok(meetings) => {
                let meeting_count = meetings.len();
                let result = import.add_meetings(meetings).await;

                if let Err(e) = result {
                    println!("Failed to add meetings to the staging: {}", e);
                } else {
                    println!("Added {meeting_count} meetings to the staging");
                }
            }
        }
    }
}

async fn sync_index(index: &mut index::MeetingIndex) -> Result<(), index::IndexError> {
    let mut import = index.start_import().await?;
    import.remove_old_meetings().await?;

    let (tx, rx) = channel(1024);
    join!(
        source::fetch_all_meetings(tx),
        add_meetings_to_index(rx, &mut import)
    );

    import.commit().await?;
    println!("Committed the staging to the database");

    Ok(())
}
