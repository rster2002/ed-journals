//! Fired when the player books a drop-ship at Frontline Solutions.

use serde::{Deserialize, Serialize};

/// Fired when the player books a drop-ship at Frontline Solutions.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct BookDropshipEvent {
    /// The number of credits the player paid for the booking.
    pub cost: u64,

    /// The name of the system where the drop-ship will travel to.
    pub destination_system: String,

    /// The name of the location where the drop-ship will travel to.
    pub destination_location: String,
}
