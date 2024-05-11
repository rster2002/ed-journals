use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FuelStatus {
    pub fuel_main: f32,
    pub fuel_reservoir: f32,
}
