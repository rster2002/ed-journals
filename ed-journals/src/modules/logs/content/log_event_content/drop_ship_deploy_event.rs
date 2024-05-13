use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DropShipDeployEvent {
    pub star_system: String,
    pub system_address: u64,
    pub body: String,
    pub body_id: u8,

    #[serde(default)]
    pub on_station: bool,

    #[serde(default)]
    pub on_planet: bool,
}
