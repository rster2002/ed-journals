use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LaunchSRVEvent {
    // TODO replace this with an enum
    #[serde(rename = "SRVType")]
    pub srv_type: String,

    #[serde(rename = "SRVType_Localised")]
    pub srv_type_localised: String,

    // TODO check if this can be replaced with an enum
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u8,
    pub player_controlled: bool,
}
