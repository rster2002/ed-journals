use ed_journals::fs::LogDir;
use ed_journals::io::LogIter;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let mut dir = LogDir::new("./test-files/journals");

    if let Some(last) = dir.last_n(1) {
        let file = File::open(last.unwrap()).unwrap();
        let buf_reader = BufReader::new(file);
        let events = LogIter::new(buf_reader);

        for event in events {
            println!("{:?}", event.unwrap());
        }
    }
}
