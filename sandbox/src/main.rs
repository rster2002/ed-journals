use std::env::current_dir;
use std::fs;
use std::path::PathBuf;
use tokio::fs::File;
use ed_journals::journal::asynchronous::LiveJournalDirReader;
use ed_journals::journal::JournalEvent;
use ed_journals::logs::asynchronous::{LiveLogDirReader, LogDirReader};
use ed_journals::state::GameState;
use ed_journals::status::blocking::StatusFileWatcher;

#[tokio::main]
async fn main() {
    let mut log_reader = LogDirReader::open("../test-journals");

    let mut state = GameState::new();

    let mut count = 0;
    while let Some(entry) = log_reader.next().await  {
        state.feed_log_event(&entry.unwrap());
        count += 1;

        // dbg!(count);
    }

    dbg!("Hi");

    let string = serde_json::to_string_pretty(&state)
        .unwrap();

    dbg!("Here");

    fs::write("temp.json", string)
        .unwrap();

    // let mut journal_watcher = LiveJournalDirReader::open("../test-journals")
    //     .unwrap();

    //
    // while let Some(entry) = journal_watcher.next().await  {
    //     match entry.unwrap() {
    //         JournalEvent::LogEvent(event) => {
    //             state.feed_log_event(&event);
    //         }
    //         JournalEvent::StatusEvent(_) => {}
    //     }
    // }

    // let watcher = StatusFileWatcher::new(PathBuf::from("./log-dir/Status.json"))
    //     .unwrap();
    //
    // for update in watcher {
    //     dbg!(update);
    // }

    // let status = ed_journals::status::blocking::read_status_file("./log-dir/Status.json")
    //     .unwrap();
    //
    // dbg!(status);

    // let dir_path = current_dir()
    //     .unwrap()
    //     .join("log-dir");
    //     // .join("Journal.2024-01-01T154931.01.log");
    // //
    // // let file = File::open()
    // //     .await
    // //     .unwrap();
    //
    // // let mut reader = LiveJournalFileReader::create(dir_path.join("Journal.2024-01-01T154931.01.log"))
    // //     .await
    // //     .unwrap();
    // //
    // let mut reader = LiveLogDirReader::open(dir_path)
    //     .unwrap();
    //
    // while let Some(entry) = reader.next().await {
    //     dbg!(entry);
    // }
}
