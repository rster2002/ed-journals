//! Fired when selling multiple entries of exploration data.

use serde::{Deserialize, Serialize};

/// Fired when selling multiple entries of exploration data.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MultiSellExplorationDataEvent {
    /// The base value of the exploration data in credits.
    pub base_value: u64,

    /// Any bonus earnings in credits.
    pub bonus: u64,

    /// The total number of credits received.
    pub total_earnings: u64,

    /// List of discoveries attributed to the current player.
    pub discovered: Vec<MultiSellExplorationDataEventDiscovery>,
}

/// Discovery that has been attributed to the current player.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MultiSellExplorationDataEventDiscovery {
    /// The name of the discovered system.
    pub system_name: String,

    /// The number of bodies that have been discovered in the system.
    #[serde(rename = "NumBodies")]
    pub number_of_bodies: u8,
}
