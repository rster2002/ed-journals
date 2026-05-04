//! Fired when the player approaches a planetary settlement.

use serde::{Deserialize, Serialize};

use crate::modules::station::StationInfo;

/// Fired when the player approaches a planetary settlement.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ApproachSettlementEvent {
    /// The name of the settlement the player is currently approaching.
    pub name: String,

    /// The localized name of the settlement. This is not always set, but might be set if the name
    /// field contains a specific format which would usually be formatted by the game.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// Station info like for example the market id, though not all settlements have this
    /// information as the game also fires this event when approaching settlements the player cannot
    /// dock at.
    #[serde(flatten)]
    pub station_info: Option<StationInfo>,

    /// The system address where the settlement is located.
    pub system_address: u64,

    /// The body id of the planet the settlement is located at.
    #[serde(rename = "BodyID")]
    pub body_id: u8,

    /// The name of the planet the settlement is located at.
    pub body_name: String,

    #[serde(flatten)]
    pub location: Option<ApproachSettlementEventLocation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct ApproachSettlementEventLocation {
    /// The latitude of the settlement on the planet.
    pub latitude: f32,

    /// The longitude of the settlement on the planet.
    pub longitude: f32,
}
