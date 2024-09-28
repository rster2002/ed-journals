use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

/// Fired when ejecting cargo out of the player's ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EjectCargoEvent {
    /// The commodity the player ejected.
    #[serde(rename = "Type")]
    pub kind: Commodity,

    /// The localized name of the commodity the player ejected.
    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,

    /// The number of commodities that were ejected.
    pub count: u16,

    /// Whether the ejected commodities were abandoned.
    pub abandoned: bool,
}
