use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct QuitACrewEvent {
    pub captain: String,

    #[serde(default)]
    pub telepresence: bool,
}
