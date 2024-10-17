use crate::civilization::Crime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeVictimEvent {
    pub offender: String,
    pub crime_type: Crime,
    pub fine: Option<u64>,
    pub bounty: Option<u64>,
}
