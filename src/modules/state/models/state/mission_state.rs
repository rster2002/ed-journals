use crate::logs::LogEvent;
use crate::state::models::resolvers::mission_state_resolver::MissionStateResolver;
use crate::state::models::state_container::StateContainer;

pub type MissionState = StateContainer<MissionStateResolver, LogEvent>;
