use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SupercruiseDestinationDropEvent {
    #[serde(rename = "Type")]
    pub kind: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: String,
    pub threat: u8,
}
