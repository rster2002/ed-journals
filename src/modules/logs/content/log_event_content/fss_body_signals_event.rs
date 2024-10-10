use serde::{Deserialize, Serialize};

use crate::modules::exploration::PlanetarySignalType;

/// Fired when the given body has any special signals. Discovering these could be through proximity
/// or through the FSS.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalsEvent {
    /// The name of the body the signals are at.
    pub body_name: String,

    /// The id of the body the signals are at.
    #[serde(rename = "BodyID")]
    pub body_id: u8,

    /// The address of the system the body is located in.
    pub system_address: u64,

    /// List of signals discovered at the given body.
    pub signals: Vec<FSSBodySignalEventSignal>,
}

/// A signal discovered at a given body.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSBodySignalEventSignal {
    /// The kind of signal that was discovered.
    #[serde(rename = "Type")]
    pub kind: PlanetarySignalType,

    /// The localized name of the signal that was discovered.
    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,

    /// The number of signals that were discovered for the given type.
    pub count: u8,
}
