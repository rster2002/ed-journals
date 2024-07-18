use crate::logs::LogEvent;
use crate::state::models::resolvers::log_state_resolver::LogStateResolver;
use crate::state::models::state_container::StateContainer;

pub type LogState = StateContainer<LogStateResolver, LogEvent>;

impl LogState {
    pub fn new() -> Self {
        LogState::default()
    }
}
