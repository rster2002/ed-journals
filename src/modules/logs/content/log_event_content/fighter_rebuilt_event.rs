use serde::{Serialize, Deserialize};

use crate::modules::models::ship::fighter_loadout::FighterLoadout;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FighterRebuiltEvent {
    pub loadout: FighterLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
}
