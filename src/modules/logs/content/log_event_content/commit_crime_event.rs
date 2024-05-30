use serde::{Deserialize, Serialize};
use crate::civilization::Crime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CommitCrimeEvent {
    pub crime_type: Crime,
    pub faction: String,
    pub fine: Option<u64>,
    pub bounty: Option<u64>,
}
