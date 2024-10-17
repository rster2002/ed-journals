use serde::{Deserialize, Serialize};

/// Fired when the player has scanned discovered all the bodies for a given system. Discovering a
/// body could either be through proximity or using the FSS.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSAllBodiesFoundEvent {
    /// The name of the current system.
    pub system_name: String,

    /// The address of the current system.
    pub system_address: u64,

    /// The number of bodies discovered.
    pub count: u8,
}
