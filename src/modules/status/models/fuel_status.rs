use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FuelStatus {
    pub fuel_main: f32,
    pub fuel_reservoir: f32,
}
