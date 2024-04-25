use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct EscapeInterdictionEvent {
    pub interdictor: String,

    #[serde(default)]
    pub is_player: bool,

    #[serde(default)]
    pub is_thargoid: bool,
}
