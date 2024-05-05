//! # ED Journals
//!
//! This library provides models and utilities to work with Elite Dangerous journal files.
//!
//! > **Warning** this project is currently in version 0.1.0, which means that it is very much work in progress. Breaking
//! > changes are likely to happen.
//!
//! ## Getting started
//!
//! Currently, the only files that are parsed are the log files. Models for working `Market.json`
//! and `Status.json` etc will be added in the future. Best place to get started is the [JournalDir]
//! model.

pub use models::journal_dir::JournalDir;
pub use models::journal_event::JournalEvent;
pub use models::journal_event_content;
pub use models::journal_event_content::JournalEventContent;
pub use models::journal_event_content::JournalEventContentKind;
pub use models::journal_file::JournalFile;
pub use models::journal_reader::JournalReader;

mod macros;
mod models;

#[cfg(test)]
mod tests {
    use std::env::current_dir;

    use crate::models::journal_dir::JournalDir;
    use crate::models::journal_event_content::JournalEventContent;
    use crate::models::journal_event_content::scan_event::ScanEventKind;

    #[test]
    fn sandbox() {
        let dir_path = current_dir()
            .unwrap()
            .parent()
            .unwrap()
            .join("test-journals");

        let log_dir = JournalDir::try_from(dir_path).unwrap();

        // let mut unique = HashSet::new();
        // let mut unique2 = HashSet::new();

        for journal in log_dir.journal_logs().unwrap() {
            let reader = journal.create_reader().unwrap();

            for entry in reader {
                match entry.unwrap().content {
                    // JournalEventContent::FileHeader(header) => {
                    //     dbg!(header.game_version);
                    // },
                    JournalEventContent::Scan(scan) => {
                        match scan.kind {
                            ScanEventKind::Star(_) => {}
                            ScanEventKind::Planet(planet) => {
                                // unique.insert(planet.atmosphere);
                                //
                                // match planet.atmosphere_type {
                                //     None => {}
                                //     Some(value) => {
                                //         unique2.insert(value);
                                //     }
                                // }
                            }
                            ScanEventKind::BeltCluster(_) => {}
                        }
                    }
                    _ => {}
                }
            }
        }

        // dbg!(unique);
        // dbg!(unique2);
    }
}
