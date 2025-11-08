//! Fired when the whole multi-crew session has ended.

use serde::{Deserialize, Serialize};

/// Fired when the whole multi-crew session has ended.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct EndCrewSessionEvent {
    #[serde(default)]
    pub on_crime: bool,

    #[serde(default)]
    pub telepresence: bool,
}
