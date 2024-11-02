//! Fired when the player 'honks' in a system.

use serde::{Deserialize, Serialize};

/// Fired when the player 'honks' in a system.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiscoveryScanEvent {
    /// The system address for the current system.
    pub system_address: u64,

    /// The number of bodies in the system.
    pub bodies: u8,
}
