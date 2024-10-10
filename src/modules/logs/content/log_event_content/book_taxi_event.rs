//! Fired when the player books a taxi at Apex Interstellar Transport.

use serde::{Deserialize, Serialize};

/// Fired when the player books a taxi at Apex Interstellar Transport.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BookTaxiEvent {
    /// The number of credits the player paid for the booking.
    pub cost: u64,

    /// The name of the system where the taxi will travel to.
    pub destination_system: String,

    /// The name of the location where the taxi will travel to.
    pub destination_location: String,

    /// Whether the booking is considered a retreat.
    #[serde(default)]
    pub retreat: bool,
}
