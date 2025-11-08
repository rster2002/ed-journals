use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct TouchdownEvent {
    pub player_controlled: bool,
    pub taxi: bool,
    pub multicrew: bool,
    pub star_system: String,
    pub system_address: u64,
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub on_station: bool,
    pub on_planet: bool,

    #[serde(flatten)]
    pub location: Option<TouchdownEventPosition>,

    pub nearest_destination: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct TouchdownEventPosition {
    pub latitude: f32,
    pub longitude: f32,
}
