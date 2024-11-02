//! Fired when the player docks their SRV to their ship.

use serde::{Deserialize, Serialize};

use crate::modules::ship::SRVType;

/// Fired when the player docks their SRV to their ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockSRVEvent {
    /// The type of SRV the player was driving.
    #[serde(rename = "SRVType", default)]
    pub srv_type: SRVType,

    /// The localized name of the type of SRV.
    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localized: Option<String>,

    #[serde(rename = "ID")]
    pub id: u8,
}
