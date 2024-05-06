use std::env::current_dir;
use ed_journals::LiveJournalFileReader;

fn main() {
    let path = current_dir()
        .unwrap()
        .join("test.log");

    let live_reader = LiveJournalFileReader::new(path)
        .unwrap();

    for entry in live_reader {
        dbg!(entry);
    }
}
