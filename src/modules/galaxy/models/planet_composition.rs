use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PlanetComposition {
    pub ice: f32,
    pub rock: f32,
    pub metal: f32,
}
