use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewMemberQuitsEvent {
    pub name: String,

    #[serde(default)]
    pub telepresence: bool,
}
