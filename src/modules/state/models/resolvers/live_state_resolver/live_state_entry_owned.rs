use serde::{Deserialize, Serialize};
use crate::state::LiveState;

#[derive(Serialize, Deserialize)]
pub struct LiveStateEntryOwned {
    pub name: String,
    pub state: LiveState,
}