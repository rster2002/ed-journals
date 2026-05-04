use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct LegacyTouchdownEvent {
    pub player_controlled: bool,

    #[serde(default)]
    pub taxi: bool,

    #[serde(default)]
    pub multicrew: bool,
    pub star_system: Option<String>,
    pub system_address: Option<u64>,
    pub body: Option<String>,

    #[serde(rename = "BodyID")]
    pub body_id: Option<u8>,

    #[serde(default)]
    pub on_station: bool,

    #[serde(default)]
    pub on_planet: bool,

    #[serde(flatten)]
    pub location: Option<LegacyTouchdownEventPosition>,

    pub nearest_destination: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct LegacyTouchdownEventPosition {
    pub latitude: f32,
    pub longitude: f32,
}
