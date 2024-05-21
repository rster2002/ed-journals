use serde::{Deserialize, Serialize};

use crate::modules::ship::FighterLoadout;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchFighterEvent {
    pub loadout: FighterLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
    pub player_controlled: bool,
}
