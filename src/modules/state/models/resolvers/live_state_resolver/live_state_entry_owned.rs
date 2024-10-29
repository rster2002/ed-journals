use crate::state::LiveState;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LiveStateEntryOwned {
    pub name: String,
    pub state: LiveState,
}
