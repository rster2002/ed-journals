use serde::{Deserialize, Serialize};

use crate::modules::commander::{
    CombatRank, EmpireRank, ExobiologyRank, ExplorationRank, FederationRank, MercenaryRank,
    TradeRank,
};

/// Fired when the player is promoted for a given rank.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum PromotionEvent {
    Exobiologist(ExobiologyRank),
    Combat(CombatRank),
    
    #[serde(rename = "Soldier")]
    Mercenary(MercenaryRank),
    Trade(TradeRank),

    #[serde(rename = "Explore")]
    Exploration(ExplorationRank),

    Federation(FederationRank),
    Empire(EmpireRank),
}
