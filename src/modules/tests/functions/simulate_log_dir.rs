use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::thread::{sleep, spawn};
use std::time::Duration;
use crate::tests::{test_dir, test_root};
const FILES: &[(&str, &str)] = &[
    ("Journal.2000-01-01T100000.01.log", include_str!("../fixtures/Journal.2000-01-01T100000.01.log")),
    ("Journal.2022-11-01T182557.01.log", include_str!("../fixtures/Journal.2022-11-01T182557.01.log")),
    ("Journal.2023-01-10T133420.01.log", include_str!("../fixtures/Journal.2023-01-10T133420.01.log")),
];

pub fn simulate_log_dir(name: &str) -> PathBuf {
    let dir_root = test_dir().path().join(name).to_path_buf();

    if dir_root.exists() {
        fs::remove_dir_all(&dir_root).unwrap();
    }

    fs::create_dir_all(&dir_root).unwrap();

    let local_root = dir_root.clone();

    spawn(move || {
        for (file_name, contents) in FILES {
            let file_path = local_root.join(file_name);

            let file = File::create(&file_path).unwrap();
            let mut buf_writer = BufWriter::new(file);

            for line in contents.lines() {
                buf_writer.write(line.as_ref()).unwrap();
                buf_writer.write(&[b'\n']).unwrap();

                buf_writer.flush().unwrap();

                sleep(Duration::from_millis(100));
            }

            sleep(Duration::from_secs(1));
        }

        sleep(Duration::from_millis(10000));
        panic!("Test should have exited by now");
    });

    dir_root
}
