//! Fired when the player confirms transfers between their ship, SRV or fleet carrier.

use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// Fired when the player confirms transfers between their ship, SRV or fleet carrier.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct CargoTransferEvent {
    /// List of transfers which have been processed.
    pub transfers: Vec<CargoTransferEventTransfer>,
}

/// An transfer which the player has performed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct CargoTransferEventTransfer {
    /// The type of commodity which has been transferred.
    #[serde(rename = "Type")]
    pub kind: Commodity,

    /// The number of items which have been transferred.
    pub count: u16,

    /// Where the commodity has been transferred to.
    pub direction: CargoTransferEventTransferDirection,
}

/// Where the commodity has been transferred to.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum CargoTransferEventTransferDirection {
    /// The player transferred to their own fleet carrier.
    #[serde(rename = "tocarrier")]
    ToCarrier,

    /// The player transferred to their current ship.
    #[serde(rename = "toship")]
    ToShip,

    /// The player transferred to their active SRV.
    #[serde(rename = "tosrv")]
    ToSRV,
}

impl CargoTransferEventTransferDirection {
    /// Whether the player transferred to their carrier.
    pub fn is_to_carrier(&self) -> bool {
        matches!(self, CargoTransferEventTransferDirection::ToCarrier)
    }

    /// Whether the player transferred to their ship.
    pub fn is_to_ship(&self) -> bool {
        matches!(self, CargoTransferEventTransferDirection::ToShip)
    }

    /// Whether the player transferred to their SRV.
    pub fn is_to_srv(&self) -> bool {
        matches!(self, CargoTransferEventTransferDirection::ToSRV)
    }
}
