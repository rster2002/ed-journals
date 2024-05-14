use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    pub latitude: f32,
    pub longitude: f32,
    pub nearest_destination: Option<String>,
}
