use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DisbandedSquadronEvent {
    pub squadron_name: String,
}
