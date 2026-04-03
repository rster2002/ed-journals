use std::fs::File;
use std::io::BufReader;
use ed_journals::io::LogIter;

fn main() {
    let file = File::open("../test-files/journals/Journal.2000-01-01T100000.01.log").unwrap();
    let buf_reader = BufReader::new(file);
    let iter = LogIter::new(buf_reader);

    for event in iter {
        println!("{:?}", event.unwrap());
    }
}