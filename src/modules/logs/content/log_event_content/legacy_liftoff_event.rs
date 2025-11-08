use crate::logs::liftoff_event::LiftoffEventPosition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct LegacyLiftoffEvent {
    #[serde(flatten)]
    pub position: Option<LiftoffEventPosition>,
    pub star_system: Option<String>,
    pub system_address: Option<u64>,
    pub body: Option<String>,

    #[serde(rename = "BodyID")]
    pub body_id: Option<u8>,

    #[serde(default)]
    pub on_station: bool,

    #[serde(default)]
    pub on_planet: bool,

    #[serde(default)]
    pub multicrew: bool,
    pub nearest_destination: Option<String>,

    #[serde(default)]
    pub taxi: bool,
    pub player_controlled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LegacyLiftoffEventPosition {
    pub latitude: f32,
    pub longitude: f32,
}
