use serde::{Serialize, Deserialize};

use crate::modules::models::commander::combat_rank::CombatRank;
use crate::modules::models::commander::empire_rank::EmpireRank;
use crate::modules::models::commander::exobiology_rank::ExobiologyRank;
use crate::modules::models::commander::exploration_rank::ExplorationRank;
use crate::modules::models::commander::federation_rank::FederationRank;
use crate::modules::models::commander::mercenary_rank::MercenaryRank;
use crate::modules::models::commander::trade_rank::TradeRank;

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
