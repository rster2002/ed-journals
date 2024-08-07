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
//! let mut state = GameState::default();
//!
//! // Read all the entries from the journal logs
//! for entry in log_reader {
//!     state.feed(&entry.unwrap());
//!     # break;
//! }
//! ```

pub use models::state::log_state::LogState;
pub use models::resolvers::log_state_resolver::current_organic_progress::CurrentOrganicProgress;
pub use models::state::game_state::GameState;
pub use models::state::system_state::SystemState;
pub use models::state::planet_state::PlanetState;
pub use models::resolvers::planet_state_resolver::planet_species_entry::PlanetSpeciesEntry;
pub use models::state::carrier_state::CarrierState;
pub use models::state::materials_state::MaterialsState;
pub use models::state::mission_state::MissionState;
pub use models::state::live_state::LiveState;
pub use models::resolvers::live_state_resolver::organic_location::OrganicLocation;
pub use models::resolvers::live_state_resolver::touchdown_location::TouchdownLocation;
pub use models::state::shipyard_state::ShipyardState;
pub use models::resolvers::shipyard_state_resolver::ship_entry::ShipEntry;
pub use models::resolvers::shipyard_state_resolver::ship_entry::ShipStatus;
pub use models::resolvers::shipyard_state_resolver::ship_entry::TransferringShipStatus;

pub use models::state_container::StateContainer;

mod models;
mod macros;
mod traits;

mod tests {
    use crate::state::LogState;

    fn test() {
        let i = LogState::default();
    }
}

