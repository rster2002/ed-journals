use std::env::current_dir;
use ed_journals::{LiveJournalDirReader, LiveJournalFileReader};

fn main() {
    let dir_path = current_dir()
        .unwrap()
        .join("log-dir");

    let live_reader = LiveJournalDirReader::new(dir_path)
        .unwrap();

    for entry in live_reader {
        dbg!(entry);
    }
}
