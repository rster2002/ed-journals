//! This event is fired when something changes in the `NavRoute.json` file and does not contain the route in the event.

use serde::{Deserialize, Serialize};

/// This event is fired when something changes in the `NavRoute.json` file and does not contain the route in the event.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct NavRouteEvent {}
