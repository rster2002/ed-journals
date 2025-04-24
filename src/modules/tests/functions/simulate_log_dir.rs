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

            sleep(Duration::from_millis(100));
            for line in contents.lines() {
                buf_writer.write(line.as_ref()).unwrap();
                buf_writer.write(&[b'\n']).unwrap();

                buf_writer.flush().unwrap();

                sleep(Duration::from_millis(50));
            }

            dbg!("Here");

            sleep(Duration::from_secs(1));
        }

        sleep(Duration::from_millis(10000));
        panic!("Test should have exited by now");
    });

    dir_root
}

// Ah yes, the tests for the test utility
#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::modules::tests::simulate_log_dir;

    #[test]
    fn generates_log_dir() {
        let path = simulate_log_dir("generates_log_dir");

        sleep(Duration::from_secs(5));

        assert!(path.exists());
    }

    #[test]
    fn contains_one_file_after_1_second() {
        let path = simulate_log_dir("contains_one_file_after_1_second");

        sleep(Duration::from_secs(1));

        assert!(path.exists());
        assert_eq!(path.read_dir().unwrap().count(), 1);
    }

    #[test]
    fn contains_partial_data_after_1_second() {
        let path = simulate_log_dir("contains_partial_data_after_1_second");

        sleep(Duration::from_millis(12 * 50));

        assert!(path.exists());

        let file_entry = path.read_dir()
            .unwrap()
            .next()
            .unwrap()
            .unwrap();

        let file_contents = read_to_string(file_entry.path()).unwrap();
        let lines = file_contents.lines()
            .count();

        assert!(lines > 8);
        assert!(lines < 15);
    }

    #[test]
    fn contains_three_files_after_15_seconds() {
        let path = simulate_log_dir("contains_three_files_after_15_seconds");

        sleep(Duration::from_secs(15));

        assert!(path.exists());
        assert_eq!(path.read_dir().unwrap().count(), 3);
    }

    #[test]
    #[should_panic]
    fn should_panic_before_25_seconds() {
        let path = simulate_log_dir("should_panic_before_25_seconds");

        sleep(Duration::from_secs(30));
    }
}
