use serde::Deserialize;

/// Part of the status that includes information about the planet that the player is close to.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PlanetStatus {
    /// The current latitude position of the player for the nearest planet.
    pub latitude: f32,

    /// The current longitude position of the player for the nearest planet.
    pub longitude: f32,

    /// This is set to [None] if the player is currently on-foot.
    pub altitude: Option<f32>,

    /// The heading of the player in degrees, going from 0 to 360.
    pub heading: f32,

    /// The name of the body the player is currently close to.
    pub body_name: String,

    /// The radius of the planet in meters.
    pub planet_radius: f32,
}
