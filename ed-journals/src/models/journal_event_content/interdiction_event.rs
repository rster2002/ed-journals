use crate::models::journal_event_content::shared::commander::combat_rank::CombatRank;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InterdictionEvent {
    pub success: bool,
    pub is_player: bool,

    // TODO not sure if this is ever filled
    pub interdicted: Option<String>,
    pub combat_rank: Option<CombatRank>,

    pub faction: Option<String>,
    pub power: Option<String>,
}
