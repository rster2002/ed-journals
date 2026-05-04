use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct OrbitInfo {
    /// Orbital radius in meters (why?!)
    pub semi_major_axis: f32,
    pub eccentricity: f32,
    pub orbital_inclination: f32,
    pub periapsis: f32,
    pub orbital_period: f32,
    pub ascending_node: Option<f32>,
    pub mean_anomaly: Option<f32>,
}
