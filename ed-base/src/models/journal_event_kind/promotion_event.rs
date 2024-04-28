use serde::Deserialize;
use crate::models::journal_event_kind::shared::commander::combat_rank::CombatRank;
use crate::models::journal_event_kind::shared::commander::empire_rank::EmpireRank;
use crate::models::journal_event_kind::shared::commander::exobiology_rank::ExobiologyRank;
use crate::models::journal_event_kind::shared::commander::exploration_rank::ExplorationRank;
use crate::models::journal_event_kind::shared::commander::federation_rank::FederationRank;
use crate::models::journal_event_kind::shared::commander::mercenary_rank::MercenaryRank;
use crate::models::journal_event_kind::shared::commander::trade_rank::TradeRank;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
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
