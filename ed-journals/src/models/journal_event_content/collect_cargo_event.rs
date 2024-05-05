use serde::Deserialize;

use crate::models::journal_event_content::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CollectCargoEvent {
    #[serde(rename = "Type")]
    pub kind: Commodity,

    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,

    pub stolen: bool,
}
