use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FighterDestroyedEvent {
    #[serde(rename = "ID")]
    pub id: u8,
}
