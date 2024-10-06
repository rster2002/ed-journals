use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipType};

/// Fired when the player initiates a module transfer to move a module from another station to the
/// station the player is currently docked at.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FetchRemoteModuleEvent {
    /// The storage slot number of the stored module.
    pub storage_slot: u8,

    /// The module being transferred.
    pub stored_item: ShipModule,

    /// The localized name of the module being transferred.
    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: Option<String>,

    /// The server id the module is coming from.
    pub server_id: u64,

    /// The number of credits it cost to transfer the module to the current location.
    pub transfer_cost: u64,

    /// The time it takes to transfer the module between locations in seconds.
    pub transfer_time: u64,

    /// The current active ship for the player.
    pub ship: ShipType,

    /// The id of the current active ship for the player.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}
