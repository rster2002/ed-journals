use crate::civilization::{Engineer, EngineerError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum EngineerProgressEvent {
    Startup(EngineerProgressStartup),
    Update(EngineerProgressUpdate),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressStartup {
    pub engineers: Vec<EngineerProgressStartupEntry>,
}

// TODO the data for this struct is so inconsistent, it could use some work.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressStartupEntry {
    pub engineer: Option<String>,

    #[serde(rename = "EngineerID")]
    pub engineer_id: Option<Engineer>,

    // TODO somehow this is optional even when the [rank] field is present? Why Frontier?!
    pub progress: Option<EngineerProgressStartupProgress>,
    pub rank: Option<u8>,
    pub rank_progress: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum EngineerProgressStartupProgress {
    Unlocked,
    Known,
    Invited,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerProgressUpdate {
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: Engineer,

    // TODO somehow this is optional even when the [rank] field is present? Why Frontier?!
    pub progress: Option<EngineerProgressStartupProgress>,
    pub rank: Option<u8>,
    pub rank_progress: Option<f32>,
}
