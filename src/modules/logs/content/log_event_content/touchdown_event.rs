use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TouchdownEvent {
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
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub nearest_destination: Option<String>,
}
