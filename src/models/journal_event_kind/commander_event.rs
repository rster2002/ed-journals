use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CommanderEvent {
    #[serde(rename = "FID")]
    pub frontier_id: String,

    #[serde(rename = "Name")]
    pub name: String,
}
