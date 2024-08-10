use crate::logs::LogEvent;
use crate::state::models::resolvers::game_state_resolver::GameStateResolver;
use crate::state::models::state_container::StateContainer;

/// The complete state of the whole game. This includes potentially the different commanders that
/// use the same game installation. By feeding the state entries from the journal log files it
/// creates a state which makes it easier to read information about the game.
pub type GameState = StateContainer<GameStateResolver, LogEvent>;
