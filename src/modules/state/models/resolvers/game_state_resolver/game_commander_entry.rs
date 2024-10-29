use serde::Serialize;
use crate::state::LogState;

#[derive(Default, Serialize)]
pub struct GameCommanderEntry {
    pub name: String,
    pub log_state: LogState,
}