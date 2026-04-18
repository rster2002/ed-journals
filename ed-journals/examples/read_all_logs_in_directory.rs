use ed_journals::fs::LogDir;
use ed_journals::io::LogIter;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let mut dir = LogDir::new("./test-files/journals");

    for path in dir {
        let file = File::open(path.unwrap()).unwrap();
        let buf_reader = BufReader::new(file);
        let events = LogIter::new(buf_reader);

        for event in events {
            println!("{:?}", event.unwrap());
        }
    }
}
