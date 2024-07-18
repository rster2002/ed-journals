use crate::logs::LogEvent;
use crate::state::models::resolvers::system_state_resolver::SystemStateResolver;
use crate::state::models::state_container::StateContainer;

pub type SystemState = StateContainer<SystemStateResolver, LogEvent>;
