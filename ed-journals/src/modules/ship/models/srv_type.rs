use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
pub enum SRVType {
    #[default]
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
