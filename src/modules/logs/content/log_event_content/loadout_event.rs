//! Fired when there are updates to the player's ship modules.

use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipSlot, ShipType};

/// Fired when there are updates to the player's ship modules.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEvent {
    /// The current active ship.
    pub ship: ShipType,

    /// The current active ship id.
    #[serde(rename = "ShipID")]
    pub ship_id: u32,

    /// The name of the current active ship.
    pub ship_name: String,

    /// The name of the current call-sign.
    pub ship_ident: String,

    /// List of current modules.
    pub modules: Vec<LoadoutEventModule>,
}

/// An active loadout module.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEventModule {
    /// The slot the module is assigned to.
    pub slot: ShipSlot,

    /// The assigned module.
    pub item: ShipModule,

    /// Whether the module is currently active.
    pub on: bool,

    /// The power priority.
    pub priority: u8,

    /// The current health for the module.
    pub health: f32,

    // TODO check when this value is used
    pub value: Option<u32>,
    pub ammo_in_clip: Option<u32>,
}
