//! Fired when the player contributes commodities to a Construction Depot.

use crate::trading::Commodity;
use serde::{Deserialize, Serialize};

/// Fired when the player contributes commodities to a Construction Depot.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ColonisationContributionEvent {
    /// The market id of the contribution target.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The contributions submitted in this event.
    #[serde(default)]
    pub contributions: Vec<ColonisationCommodityContribution>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ColonisationCommodityContribution {
    /// Internal name of the contribution
    pub name: Commodity,

    /// The localized name of the commodity the player contributed.
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    /// Amount of resources contributed
    pub amount: u16,
}
