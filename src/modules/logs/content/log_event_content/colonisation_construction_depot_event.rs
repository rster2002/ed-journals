//! Fired when a player receives an update on the required commodities for a construction depot.
//! This often fires when docking on a System Colonisation Ship, or after they have contributed.

use serde::{Deserialize, Serialize};

/// Fired when a player receives an update on the required commodities for a construction depot.
/// This often fires when docking on a System Colonisation Ship, or after they have contributed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ColonisationConstructionDepotEvent {
    /// The market id of the constribution target.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The contributions submitted in this event.
    #[serde(default)]
    pub resources_required: Vec<ColonisationConstructionDepotRequiredResource>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ColonisationConstructionDepotRequiredResource {
    /// Internal name of the required commodity.
    pub name: String,

    /// The localized name of the commodity required.
    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    /// Amount of commodities required.
    pub required_amount: u32,

    /// Amount of commodities provided.
    pub provided_amount: u32,

    /// Payment in credits that will be awarded for providing a single unit of this resource.
    pub payment: u32,
}
