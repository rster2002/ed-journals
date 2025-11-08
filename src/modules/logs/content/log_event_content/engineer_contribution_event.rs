//! Fired when the player contributes to an unlock requirement for an engineer.

use crate::civilization::Engineer;
use crate::modules::materials::Material;
use crate::modules::trading::Commodity;
use serde::{Deserialize, Serialize};

/// Fired when the player contributes to an unlock requirement for an engineer.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEvent {
    /// The engineer the player contributed to.
    pub engineer: String,

    /// The ID of the engineer.
    #[serde(rename = "EngineerID")]
    pub engineer_id: Engineer,

    /// The kind of contribution the player has made to the engineer.
    #[serde(rename = "Type", flatten)]
    pub kind: EngineerContributionEventType,
}

/// The kind of contribution to an engineer.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase", tag = "Type")]
pub enum EngineerContributionEventType {
    /// A commodity contribution where the player sourced and delivered a certain commodity.
    Commodity(EngineerContributionEventCommodityContribution),

    /// A material contribution where the player donated a number of materials.
    Materials(EngineerContributionEventMaterialContribution),

    /// A bounty contribution where the player has handed in bounties.
    Bounty(EngineerContributionEventBountyContribution),
}

/// Contribution where the player sourced and delivered a certain commodity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEventCommodityContribution {
    /// The commodity that was delivered.
    pub commodity: Commodity,

    /// The number of commodities that were delivered at this moment.
    pub quantity: u16,

    /// The number of commodities that player has delivered to the engineer in total.
    pub total_quantity: u16,
}

/// Contribution where the player donated a number of materials.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEventMaterialContribution {
    /// The material that was donated.
    pub material: Material,

    /// The number of materials that were donated at this moment.
    pub quantity: u16,

    /// The number of materials that the player has donated to the engineer in total.
    pub total_quantity: u16,
}

/// Contribution where the plater handed in a number of bounties.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEventBountyContribution {
    /// Number of bounties that were handed in at this moment.
    pub quantity: u32,

    /// The total number of bounties the player has handed in at this engineer so far.
    pub total_quantity: u32,
}
