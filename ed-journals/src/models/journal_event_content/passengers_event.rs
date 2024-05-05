use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersEvent {
    pub manifest: Vec<PassengersManifestEntry>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersManifestEntry {
    #[serde(rename = "MissionID")]
    mission_id: u64,

    // TODO replace with enum
    #[serde(rename = "Type")]
    kind: String,

    #[serde(rename = "VIP")]
    vip: bool,
    wanted: bool,
    count: u8,
}
