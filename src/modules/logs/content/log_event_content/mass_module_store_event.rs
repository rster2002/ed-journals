//! Fired when storing multiple modules at the current location at once.

use serde::{Deserialize, Serialize};

use crate::modules::ship::{Blueprint, ShipModule, ShipSlot, ShipType};

/// Fired when storing multiple modules at the current location at once.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEvent {
    /// The market id where the modules are getting stored.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The type of ship where the modules are getting stored from.
    pub ship: ShipType,

    /// The id of the ship the modules are getting stored from.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    /// List of modules which have been stored.
    pub items: Vec<MassModuleStoreEventItem>,
}

/// A module that has been stored at the current location.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEventItem {
    /// The slot the module is coming from.
    pub slot: ShipSlot,

    /// The kind of module stored.
    pub name: ShipModule,

    /// Whether the module is hot.
    pub hot: bool,

    /// Engineering modification applied to the module, if any.
    #[serde(flatten)]
    pub engineering: Option<MassModuleStoreEventModification>,
}

/// Engineering modification applied to a module.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MassModuleStoreEventModification {
    /// The kind of blueprint applied.
    pub engineer_modification: Blueprint,

    /// The level or 'grade' of the modification.
    pub level: u8,

    /// The quality or 'progress' of the current modification.
    pub quality: u8,
}
