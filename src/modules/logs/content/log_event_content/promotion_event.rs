use serde::{Serialize, Deserialize};

use crate::modules::shared::commander::combat_rank::CombatRank;
use crate::modules::shared::commander::empire_rank::EmpireRank;
use crate::modules::shared::commander::exobiology_rank::ExobiologyRank;
use crate::modules::shared::commander::exploration_rank::ExplorationRank;
use crate::modules::shared::commander::federation_rank::FederationRank;
use crate::modules::shared::commander::mercenary_rank::MercenaryRank;
use crate::modules::shared::commander::trade_rank::TradeRank;

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
