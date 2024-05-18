//! Below is a simple example on how to read logs and feed then into the state:
//!
//! ```rust
//! use std::env::current_dir;
//! use ed_journals::logs::blocking::LogDirReader;
//! use ed_journals::state::GameState;
//!
//! let path = current_dir()
//!     .unwrap()
//!     .join("test-files")
//!     .join("journals");
//!
//! // Create a reader and an empty game state
//! let mut log_reader = LogDirReader::open(&path);
//! let mut state = GameState::new();
//!
//! // Read all the entries from the journal logs
//! for entry in log_reader {
//!     state.feed_log_event(&entry.unwrap());
//! }
//!
//! // Now we can access retrieve data from the state:
//! let current_commander = state.current_commander().unwrap();
//!
//! println!("Current commander: {}", current_commander.name);
//! println!("Nr of visited systems: {}", current_commander.systems.len());
//! ```

mod models;

pub use models::game_state::GameState;
pub use models::commander_state::CommanderState;
pub use models::system_state::SystemState;
