use tokio::{
    join,
    sync::mpsc::{channel, Receiver},
};

pub mod meeting;
pub mod source;

async fn print_meetings(mut rx: Receiver<source::FetchMeetingResult>) {
    while let Some(result) = rx.recv().await {
        match result {
            Ok(meetings) => {
                for meeting in meetings {
                    println!("{:?}", meeting);
                }
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = channel(100);

    join!(source::fetch_all_meetings(tx), print_meetings(rx));
}
