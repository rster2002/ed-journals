use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use crate::tests::test_dir;

pub fn simulate_log_file(name: &str) -> (PathBuf, JoinHandle<()>) {
    let dir_root = test_dir().path().join(name).to_path_buf();

    if dir_root.exists() {
        fs::remove_dir_all(&dir_root).unwrap();
    }

    fs::create_dir_all(&dir_root).unwrap();

    let file_path = dir_root.join("Journal.2000-01-01T100000.01.log");

    let file = File::create(&file_path).unwrap();
    let mut buf_writer = BufWriter::new(file);

    let handle = spawn(move || {
        sleep(Duration::from_secs(1));

        let contents = include_str!("../fixtures/Journal.2000-01-01T100000.01.log");

        for line in contents.lines() {
            buf_writer.write(line.as_ref()).unwrap();
            buf_writer.write(&[b'\n']).unwrap();

            buf_writer.flush().unwrap();

            sleep(Duration::from_millis(100));
        }

        sleep(Duration::from_millis(10000));
        panic!("Test should have exited by now");
    });

    (file_path, handle)
}
