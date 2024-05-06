use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrewLaunchFighterEvent {
    #[serde(rename = "ID")]
    pub id: u64,
    pub name: String,

    #[serde(default)]
    pub telepresence: bool,
}
