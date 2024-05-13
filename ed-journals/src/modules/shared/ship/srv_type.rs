use std::fmt::{Display, Formatter};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SRVType {
    #[serde(rename = "testbuggy")]
    Scarab,
}

impl Display for SRVType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SRVType::Scarab => "Scarab",
            }
        )
    }
}
