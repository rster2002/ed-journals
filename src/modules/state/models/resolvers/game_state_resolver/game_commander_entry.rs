use crate::state::LogState;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct GameCommanderEntry {
    pub name: String,
    pub log_state: LogState,
}
