use serde::Deserialize;
use crate::models::journal_event_kind::shared::ship::fighter_loadout::FighterLoadout;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FighterRebuiltEvent {
    pub loadout: FighterLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
}
