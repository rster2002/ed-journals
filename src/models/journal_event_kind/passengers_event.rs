use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersEvent {
    pub manifest: Vec<PassengersManifestEntry>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PassengersManifestEntry {
    #[serde(rename = "MissionID")]
    mission_id: String,

    #[serde(rename = "type")]
    kind: String,

    #[serde(rename = "VIP")]
    vip: bool,
    wanted: bool,
    count: u8,
}
