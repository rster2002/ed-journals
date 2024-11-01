use crate::{logs::load_game_event::LoadGameEventGameMode, state::LogState};
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct GameCommanderEntry {
    pub name: String,
    pub game_mode: Option<LoadGameEventGameMode>,
    pub log_state: LogState,
}
