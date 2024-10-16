use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SRVType {
    #[serde(rename = "testbuggy")]
    Scarab,

    #[serde(rename = "combat_multicrew_srv_01")]
    Scorpion,
}

impl Display for SRVType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SRVType::Scarab => "Scarab",
                SRVType::Scorpion => "Scorpion",
            }
        )
    }
}

impl Default for SRVType {
    fn default() -> Self {
        Self::Scarab
    }
}
