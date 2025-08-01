//! Fired when targeting a system for an FSD jump.

use serde::{Deserialize, Serialize};

use crate::modules::galaxy::StarClass;

/// Fired when targeting a system for an FSD jump. This is either when the player manually selects
/// a target or when the next destination on a route is automatically selected.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct FSDTargetEvent {
    // TODO check when this is included
    #[serde(rename = "Starsystem")]
    star_system: Option<String>,

    /// The name of the system selected.
    name: String,

    /// Number of remaining jumps on the route, if any.
    #[serde(default)]
    remaining_jumps_in_route: u32,

    /// Star class of the select system.
    star_class: StarClass,
}
