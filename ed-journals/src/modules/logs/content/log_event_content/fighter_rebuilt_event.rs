use serde::Deserialize;

use crate::modules::shared::ship::fighter_loadout::FighterLoadout;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FighterRebuiltEvent {
    pub loadout: FighterLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
}
