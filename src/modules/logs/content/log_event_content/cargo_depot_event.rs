//! Fired when the player delivers or collects commodities for a given mission.

use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// Fired when the player delivers or collects commodities for a given mission.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CargoDepotEvent {
    /// The id of the mission this event applies to.
    #[serde(rename = "MissionID")]
    pub mission_id: u64,

    /// The kind of action the player performed for this mission.
    pub update_type: CargoDepotEventUpdateType,

    /// The commodity which were collected or delivered.
    pub cargo_type: Option<Commodity>,

    /// The number of items which were collected or delivered.
    pub count: Option<u16>,

    /// The market id where the player should collect the commodities from.
    #[serde(rename = "StartMarketID")]
    pub start_market_id: u64,

    /// The market id where the player should deliver the commodities to.
    #[serde(rename = "EndMarketID")]
    pub end_market_id: u64,

    /// Total number of items which have been collected so far.
    pub items_collected: u16,

    /// Total number of items delivered so far.
    pub items_delivered: u16,

    /// The total number of items which are required for the mission to be a success.
    pub total_items_to_deliver: u16,

    /// Current progress factor where 0 is no progress and 1 is completed.
    pub progress: f32,
}

/// The kind of action the player performed for the given mission.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum CargoDepotEventUpdateType {
    /// The player collected commodities for the mission.
    Collect,

    /// The player delivered commodities for the mission.
    Deliver,

    /// Someone within the current active wing caused the mission to be updated.
    WingUpdate,
}

impl CargoDepotEventUpdateType {
    /// Whether the update was from a collect event.
    pub fn is_collection(&self) -> bool {
        matches!(self, CargoDepotEventUpdateType::Collect)
    }

    /// Whether the update was from a delivery event.
    pub fn is_delivery(&self) -> bool {
        matches!(self, CargoDepotEventUpdateType::Deliver)
    }

    /// Whether the update was from a wing update.
    pub fn is_wing_update(&self) -> bool {
        matches!(self, CargoDepotEventUpdateType::WingUpdate)
    }
}
