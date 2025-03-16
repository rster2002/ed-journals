#![cfg_attr(docsrs, feature(doc_cfg))]

//! # ED Journals
//!
//! This library provides models and utilities to work with Elite Dangerous journal files.
//!
//! > **Warning** this project is currently in beta, which means that it is very much work in progress. Breaking
//! > changes are likely to happen.
//!
//! ## Where to start
//!
//! This library contains quite a large number of modules that each cover a different part of the
//! game, but there are a couple of modules that are important to point out:
//!
//! * The [logs] module contains readers and models for reading the `Journal.log` files that are
//!   stored in the games journal directory.
//! * [State](state) can be used to turn the logs from the `Journal.log` files into a single state that can
//!   then be queried and used to figure out the current or previous state of the game.
//! * The [journal] module can be used to interact with the whole journal directory and can watch
//!   the directory as a whole for changes.

pub use modules::backpack;
pub use modules::cargo;
pub use modules::civilization;
pub use modules::commander;
pub use modules::exobiology;
pub use modules::exploration;
pub use modules::galaxy;
pub use modules::journal;
pub use modules::logs;
pub use modules::market;
pub use modules::materials;
pub use modules::mixed;
pub use modules::modules_info;
pub use modules::nav_route;
pub use modules::odyssey;
pub use modules::outfitting;
pub use modules::partials;
pub use modules::ship;
pub use modules::ship_locker;
pub use modules::shipyard;
pub use modules::state;
pub use modules::station;
pub use modules::status;
pub use modules::thargoid;
pub use modules::trading;

mod modules;

#[cfg(test)]
mod tests {
    use crate::logs::{LogEvent, LogEventContent};
    use std::env::current_dir;
    use std::fs;
    use std::hash::{DefaultHasher, Hash, Hasher};
    use std::path::{Path, PathBuf};
    use std::thread::current;
    use crate::modules::logs2::{LogDir, LogError};

    pub struct TestFile(PathBuf);

    impl TestFile {
        pub fn path(&self) -> PathBuf {
            self.0.clone()
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            fs::remove_file(&self.0).unwrap()
        }
    }

    pub struct TestDir(PathBuf);

    impl TestDir {
        pub fn path(&self) -> PathBuf {
            self.0.clone()
        }

        pub fn file(&self, number: u32) -> PathBuf {
            self.0.join(format!("{}.tmp", number))
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.0).unwrap();
        }
    }

    pub fn test_root() -> PathBuf {
        PathBuf::from("./test-files")
    }

    pub fn test_dir() -> TestDir {
        let mut hasher = DefaultHasher::new();
        current().id().hash(&mut hasher);

        let hash = hasher.finish();

        let path = test_root()
            .join("temp-dir")
            .join(format!("dir-{}", hash));

        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();

        TestDir(path)
    }

    pub fn test_file() -> TestFile {
        let temp_dir = test_root().join("temp-dir");

        fs::create_dir_all(&temp_dir).unwrap();

        let mut hasher = DefaultHasher::new();
        current().id().hash(&mut hasher);

        let hash = hasher.finish();
        TestFile(temp_dir.join(format!("test-{}", hash)))
    }

    #[test]
    fn test_journals_are_parsed_correctly() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let log_dir = LogDir::new(dir_path);

        let mut file_header_count = 0;
        let mut entry_count = 0;

        for file in log_dir.iter().unwrap() {
            for entry in file.iter().unwrap() {
                entry_count += 1;

                let entry = match entry {
                    Ok(entry) => entry,
                    Err(error) => {
                        dbg!(&file);
                        dbg!(&error);
                        panic!("{:?}", error);
                    }
                };

                if let LogEventContent::FileHeader(_) = entry.content {
                    file_header_count += 1;
                }
            }
        }

        dbg!(file_header_count);
        dbg!(entry_count);

        // assert_eq!(logs.len(), file_header_count);
    }
}
