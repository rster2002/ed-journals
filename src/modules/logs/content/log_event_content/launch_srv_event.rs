//! Fired when launching an SRV while landed on a planet.

use serde::{Deserialize, Serialize};

use crate::modules::ship::SRVType;

/// Fired when launching an SRV while landed on a planet.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchSRVEvent {
    /// The type of SRV launched.
    #[serde(rename = "SRVType")]
    pub srv_type: SRVType,

    /// The localized name of the type of SRV launched.
    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localized: String,

    // TODO check if this can be replaced with an enum
    pub loadout: String,

    /// The id of the SRV.
    #[serde(rename = "ID")]
    pub id: u8,

    /// Whether the SRV is player controlled.
    pub player_controlled: bool,
}
