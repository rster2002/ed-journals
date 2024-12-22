//! Resolves are the parts of state that accumulate the events and create usable data from them.
//! They are not made to be used directly, but instead they are wrapped by
//! [StateContainer](super::state_container::StateContainer) and type aliased.

pub mod carrier_state_resolver;
pub mod game_state_resolver;
pub mod journal_state_resolver;
pub mod live_state_resolver;
pub mod log_state_resolver;
pub mod materials_state_resolver;
pub mod mission_state_resolver;
pub mod planet_state_resolver;
pub mod shipyard_state_resolver;
pub mod system_state_resolver;
