//! Fired when the player discovers a non-body signal.

use serde::{Deserialize, Serialize};

/// Fired when the player discovers a non-body signal.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct FSSSignalDiscoveredEvent {
    /// The address of the system the signal is in.
    pub system_address: u64,

    // TODO replace with enum.
    /// The name of the signal.
    pub signal_name: String,

    /// The localized name of the signal.
    #[serde(rename = "SignalName_Localised")]
    pub signal_name_localized: Option<String>,

    /// Whether the signal is a station or a fleet carrier.
    #[serde(default)]
    pub is_station: bool,
}
