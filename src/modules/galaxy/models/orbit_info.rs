use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OrbitInfo {
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_inclination: f32,
    pub periapsis: f32, // Hopefully this is in AU
    pub orbital_period: f32,
    pub ascending_node: Option<f32>,
    pub mean_anomaly: Option<f32>,
}
