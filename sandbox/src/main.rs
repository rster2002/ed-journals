use std::env::current_dir;
use std::path::PathBuf;
use tokio::fs::File;
use ed_journals::logs::asynchronous::LiveLogDirReader;
use ed_journals::status::blocking::StatusFileWatcher;

#[tokio::main]
async fn main() {
    let watcher = StatusFileWatcher::new(PathBuf::from("./log-dir/Status.json"))
        .unwrap();

    for update in watcher {
        dbg!(update);
    }

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
