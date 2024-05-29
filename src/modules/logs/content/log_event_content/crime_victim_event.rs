use serde::{Deserialize, Serialize};
use crate::civilization::Crime;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CrimeVictimEvent {
    pub offender: String,
    pub crime_type: Crime,
    pub fine: Option<u64>,
    pub bounty: Option<u64>,
}
