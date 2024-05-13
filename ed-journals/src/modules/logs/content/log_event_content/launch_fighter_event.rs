use serde::{Serialize, Deserialize};

use crate::modules::shared::ship::fighter_loadout::FighterLoadout;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchFighterEvent {
    pub loadout: FighterLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
    pub player_controlled: bool,
}
