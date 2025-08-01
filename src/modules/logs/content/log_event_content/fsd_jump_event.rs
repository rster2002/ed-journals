//! Fired when the player jumps to another system.

use crate::galaxy::GalacticDistance;
use crate::modules::civilization::LocationInfo;
use serde::{Deserialize, Serialize};

/// Fired when the player jumps to another system. This is fired when traveling between systems.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct FSDJumpEvent {
    /// The distance the player has jumped in LY.
    pub jump_dist: GalacticDistance,

    /// The number of tonnes of fuel used for the jump.
    pub fuel_used: f32,

    /// The new fuel level after the jump.
    pub fuel_level: f32,

    /// Information about the system the player is jumping to.
    #[serde(flatten)]
    pub system_info: LocationInfo,
}
