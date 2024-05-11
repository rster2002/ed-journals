use std::env::current_dir;
use tokio::fs::File;
use ed_journals::r#async::LiveJournalDirReader;
use ed_journals::r#async::LiveJournalFileReader;
use ed_journals::r#async::JournalFileReader;

#[tokio::main]
async fn main() {
    let dir_path = current_dir()
        .unwrap()
        .join("log-dir");
        // .join("Journal.2024-01-01T154931.01.log");
    //
    // let file = File::open()
    //     .await
    //     .unwrap();

    // let mut reader = LiveJournalFileReader::create(dir_path.join("Journal.2024-01-01T154931.01.log"))
    //     .await
    //     .unwrap();
    //
    let mut reader = LiveJournalDirReader::create(dir_path)
        .unwrap();

    while let Some(entry) = reader.next().await {
        dbg!(entry);
    }
}
