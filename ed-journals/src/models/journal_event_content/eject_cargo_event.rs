use crate::models::journal_event_content::shared::trading::commodity::Commodity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EjectCargoEvent {
    #[serde(rename = "Type")]
    pub kind: Commodity,

    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,
    pub count: u16,
    pub abandoned: bool,
}
