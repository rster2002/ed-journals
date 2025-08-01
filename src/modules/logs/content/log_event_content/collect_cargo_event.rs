//! Fired when the player collects a commodity.

use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// Fired when the player collects a commodity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct CollectCargoEvent {
    /// The commodify the player has collected.
    #[serde(rename = "Type")]
    pub kind: Commodity,

    /// The localized name of the commodity the player collected.
    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,

    /// Whether the collected commodity is considered stolen.
    pub stolen: bool,
}
