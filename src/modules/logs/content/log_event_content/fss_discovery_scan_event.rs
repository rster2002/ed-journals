//! Fired when discovering a body in a given system, either through proximity or through the FSS.

use serde::{Deserialize, Serialize};

/// Fired when discovering a body in a given system, either through proximity or through the FSS.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSDiscoveryScan {
    /// Value between 0-1 to indicate percentage of system scanned
    pub progress: f32,

    /// The total number of bodies the player has scanned in the current system.
    pub body_count: u8,

    /// The total number of non-body signals the player has scanned in the current system.
    pub non_body_count: u8,

    /// The name of the current system.
    pub system_name: String,

    /// The address of the current system.
    pub system_address: u64,
}
