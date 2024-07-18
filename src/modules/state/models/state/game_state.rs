use crate::logs::LogEvent;
use crate::state::models::resolvers::log_state_resolver::LogStateResolver;
use crate::state::models::state_container::StateContainer;

pub type GameState = StateContainer<LogStateResolver, LogEvent>;

impl GameState {
    pub fn new() -> Self {
        GameState::default()
    }
}
