use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DockSRVEvent {
    // TODO replace with enum
    #[serde(rename = "SRVType")]
    pub srv_type: String,

    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localised: String,

    #[serde(rename = "ID")]
    pub id: u8,
}
