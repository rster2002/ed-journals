use crate::logs::LogEvent;
use crate::state::models::resolvers::shipyard_state_resolver::ShipyardStateResolver;
use crate::state::models::state_container::StateContainer;

pub type ShipyardState = StateContainer<ShipyardStateResolver, LogEvent>;
