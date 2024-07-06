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
//!     # break;
//! }
//! ```

pub use models::log_state::LogState;
pub use models::log_state::current_organic_progress::CurrentOrganicProgress;
pub use models::game_state::GameState;
pub use models::system_state::SystemState;
pub use models::planet_state::PlanetState;
pub use models::materials_state::MaterialsState;
pub use models::carrier_state::CarrierState;
pub use models::planet_state::planet_species_entry::PlanetSpeciesEntry;
pub use models::journal_state::JournalState;
pub use models::journal_state::current_organic_location::CurrentOrganicLocation;

mod models;
mod macros;

