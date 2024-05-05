use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DockFighterEvent {
    #[serde(rename = "ID")]
    pub id: u8,
}
