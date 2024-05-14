use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InterdictedEvent {
    pub submitted: bool,

    // TODO check when this is [None]
    pub interdictor: Option<String>,

    #[serde(default)]
    pub is_player: bool,

    #[serde(default)]
    pub is_thargoid: bool,

    // TODO check when this is [None]
    pub faction: Option<String>,
}
