use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct JoinACrewEvent {
    pub captain: String,

    #[serde(default)]
    pub telepresence: bool,
}
