//! There are multiple models that can be used to track state for both [LogEvents](crate::logs::LogEvent) and
//! [JournalEvents](crate::journal::JournalEvent). The most top-level models that track the most are:
//!
//! - [GameState] is used for when you want to track the state of an Elite: Dangerous installation.
//!   It automatically differentiates between different commanders if the installation happens to
//!   contain logs for multiple accounts.
//! - [LogState] acts a lot like GameState, but *does not* differentiate between different
//!   commanders and treats everything as a single state. You might use this for example when
//!   processing logs where the specific commander does not matter.
//! - [JournalState] is used in conjunction with a [LiveJournalDirReader] to combine log information
//!   with live information from the various JSON files that the game updates periodically.

use crate::journal::blocking::LiveJournalDirReader;

pub use models::resolvers::game_state_resolver::game_commander_entry::GameCommanderEntry;
pub use models::resolvers::journal_state_resolver::journal_commander_entry::JournalCommanderEntry;
pub use models::resolvers::live_state_resolver::live_state_entry::LiveStateEntry;
pub use models::resolvers::live_state_resolver::live_state_entry_owned::LiveStateEntryOwned;
pub use models::resolvers::live_state_resolver::organic_location::OrganicLocation;
pub use models::resolvers::live_state_resolver::touchdown_location::TouchdownLocation;
pub use models::resolvers::log_state_resolver::current_organic_progress::CurrentOrganicProgress;
pub use models::resolvers::planet_state_resolver::planet_species_entry::PlanetSpeciesEntry;
pub use models::resolvers::shipyard_state_resolver::ship_entry::ShipEntry;
pub use models::resolvers::shipyard_state_resolver::ship_entry::ShipStatus;
pub use models::resolvers::shipyard_state_resolver::ship_entry::TransferringShipStatus;
pub use models::state::carrier_state::CarrierState;
pub use models::state::game_state::GameState;
pub use models::state::journal_state::JournalState;
pub use models::state::live_state::LiveState;
pub use models::state::log_state::LogState;
pub use models::state::materials_state::MaterialsState;
pub use models::state::mission_state::MissionState;
pub use models::state::planet_state::PlanetState;
pub use models::state::shipyard_state::ShipyardState;
pub use models::state::system_state::SystemState;

pub use models::state_container::StateContainer;

/// Resolvers which are used to build up the state. You shouldn't use these directly, but instead
/// should use one of the type aliases that use [StateContainer], like [LogState].
pub use models::resolvers;

mod macros;
mod models;
mod traits;
