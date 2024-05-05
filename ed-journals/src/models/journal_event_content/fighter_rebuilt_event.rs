use crate::models::journal_event_content::shared::ship::fighter_loadout::FighterLoadout;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FighterRebuiltEvent {
    pub loadout: FighterLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
}
