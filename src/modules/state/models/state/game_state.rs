use crate::logs::LogEvent;
use crate::state::models::resolvers::game_state_resolver::GameStateResolver;
use crate::state::models::state_container::StateContainer;

/// See [GameStateResolver].
pub type GameState = StateContainer<GameStateResolver, LogEvent>;
