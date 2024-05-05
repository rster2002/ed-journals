use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct FileHeaderEvent {
    pub part: u8,
    pub language: String,

    #[serde(default, rename = "Odyssey")]
    pub odyssey: bool,

    #[serde(rename = "gameversion")]
    pub game_version: String,
    pub build: String,
}
