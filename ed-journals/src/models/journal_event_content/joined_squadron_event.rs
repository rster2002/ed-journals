use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct JoinedSquadronEvent {
    pub squadron_name: String,
}
