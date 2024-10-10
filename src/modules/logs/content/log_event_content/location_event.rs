use serde::{Deserialize, Serialize};

use crate::modules::civilization::LocationInfo;

/// Fired at times when the game loads in a location, usually when just starting the game or when
/// the player is moven between locations for example by using a fleet carrier escape pod or when
/// the player died.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationEvent {
    // TODO check when this is filled
    /// Distance from the arrival star of the current system.
    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: Option<f32>,

    /// Whether the player is currently docked.
    pub docked: bool,

    /// Whether the player is in a taxi.
    #[serde(default)]
    pub taxi: bool,

    /// Whether the player is currently in a multicrew.
    #[serde(default)]
    pub multicrew: bool,

    /// Detailed information about the current location.
    #[serde(flatten)]
    pub location_info: LocationInfo,
}
