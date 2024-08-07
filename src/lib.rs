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
pub use modules::ship;
pub use modules::ship_locker;
pub use modules::shipyard;
pub use modules::partials;
pub use modules::state;
pub use modules::station;
pub use modules::status;
pub use modules::thargoid;
pub use modules::trading;

mod modules;

#[cfg(test)]
mod tests {
    use std::env::current_dir;
    use std::path::PathBuf;
    use crate::logs::LogDir;
    use crate::logs::LogEventContent;

    pub fn test_root() -> PathBuf {
        PathBuf::from("./test-files")
    }

    #[test]
    fn test_journals_are_parsed_correctly() {
        let dir_path = current_dir().unwrap().join("test-files").join("journals");

        let log_dir = LogDir::new(dir_path);

        let logs = log_dir.journal_logs().unwrap();

        assert!(logs.len() > 10);

        let mut file_header_count = 0;
        let mut entry_count = 0;

        for journal in &logs {
            let mut found_file_header = false;
            let reader = journal.create_blocking_reader().unwrap();

            for entry in reader {
                entry_count += 1;

                if let LogEventContent::FileHeader(_) = entry.unwrap().content {
                    found_file_header = true;
                    file_header_count += 1;
                }
            }
        }

        dbg!(file_header_count);
        dbg!(entry_count);

        // assert_eq!(logs.len(), file_header_count);
    }
}
