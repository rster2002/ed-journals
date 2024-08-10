use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules_info::models::modules_info_entry::ModulesInfoEntry;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ModulesInfo {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,
    pub modules: Vec<ModulesInfoEntry>,
}
