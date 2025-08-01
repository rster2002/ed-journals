//! Fired when the player collected a commodity.

use crate::trading::Commodity;
use serde::{Deserialize, Serialize};

/// Fired when the player collected a commodity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CargoEvent {
    /// The current vessel the player is currently piloting.
    pub vessel: CargoEventVessel,

    // TODO this seems to be missing for SRV?
    /// The current inventory of the current vessel.
    #[serde(default)]
    pub inventory: Vec<CargoEventInventoryItem>,
}

/// The type of vessel the player is currently piloting.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub enum CargoEventVessel {
    /// The player has collected the cargo using their ship.
    Ship,

    /// The player has collected the cargo using their SRV.
    SRV,
}

impl CargoEventVessel {
    /// Whether the player collected the cargo using their ship.
    pub fn is_ship(&self) -> bool {
        matches!(self, CargoEventVessel::Ship)
    }

    /// Whether the player collected the cargo using their SRV.
    pub fn is_srv(&self) -> bool {
        matches!(self, CargoEventVessel::SRV)
    }
}

/// An entry for an item in the player's inventory.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CargoEventInventoryItem {
    /// The type of commodity.
    pub name: Commodity,

    /// The number of tonnes the player has for this commodity.
    pub count: u16,

    /// Whether the commodity is considered stolen.
    pub stolen: u16,
}
