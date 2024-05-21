use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EscapeInterdictionEvent {
    pub interdictor: String,

    #[serde(default)]
    pub is_player: bool,

    #[serde(default)]
    pub is_thargoid: bool,
}
