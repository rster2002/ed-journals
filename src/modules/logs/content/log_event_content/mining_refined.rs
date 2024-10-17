use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// Fired when a commodity has been refined.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MiningRefinedEvent {
    /// The kind of commodity that was refined.
    #[serde(rename = "Type")]
    pub kind: Commodity,

    /// The localized name of the refined commodity.
    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,
}
