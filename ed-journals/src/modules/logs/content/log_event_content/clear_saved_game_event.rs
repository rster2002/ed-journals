use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClearSavedGameEvent {
    pub name: String,

    #[serde(rename = "FID")]
    pub fid: String,
}
