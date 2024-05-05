use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InvitedToSquadronEvent {
    pub squadron_name: String,
}
