use serde::{Deserialize, Serialize};
use crate::commander::{CombatRank, EmpireRank, ExplorationRank, FederationRank, TradeRank};

/// Includes information about the current ranks of the player.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RankEvent {
    pub combat: CombatRank,
    pub trade: TradeRank,
    pub explore: ExplorationRank,
    pub empire: EmpireRank,
    pub federation: FederationRank,

    #[serde(rename = "CQC")]
    pub cqc: u8,
}
