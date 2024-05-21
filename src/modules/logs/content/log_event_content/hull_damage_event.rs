use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct HullDamageEvent {
    pub health: f32,
    pub player_pilot: bool,

    // TODO sometimes this is missing. Should check if it missing has special meaning
    #[serde(default)]
    pub fighter: bool,
}
