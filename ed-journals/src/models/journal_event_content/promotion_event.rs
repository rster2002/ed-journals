use crate::models::journal_event_content::shared::commander::combat_rank::CombatRank;
use crate::models::journal_event_content::shared::commander::empire_rank::EmpireRank;
use crate::models::journal_event_content::shared::commander::exobiology_rank::ExobiologyRank;
use crate::models::journal_event_content::shared::commander::exploration_rank::ExplorationRank;
use crate::models::journal_event_content::shared::commander::federation_rank::FederationRank;
use crate::models::journal_event_content::shared::commander::mercenary_rank::MercenaryRank;
use crate::models::journal_event_content::shared::commander::trade_rank::TradeRank;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
