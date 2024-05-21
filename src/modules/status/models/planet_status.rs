use serde::{Deserialize};

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PlanetStatus {
    pub latitude: f32,
    pub longitude: f32,
    pub altitude: f32,
    pub heading: f32,
    pub body_name: String,
    pub planet_radius: f32,
}
