use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileHeaderEvent {
    pub part: u8,
    pub language: String,

    #[serde(rename = "gameversion")]
    pub game_version: String,
    pub build: String,
}
