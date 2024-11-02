//! Fired when the player scans a datapoint and extracts a message from it.

use serde::{Deserialize, Serialize};

/// Fired when the player scans a datapoint and extracts a message from it.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DatalinkScanEvent {
    /// Message extracted from the datapoint.
    pub message: String,

    /// The localized message extracted from the datapoint.
    #[serde(rename = "Message_Localised")]
    pub message_localized: Option<String>,
}
