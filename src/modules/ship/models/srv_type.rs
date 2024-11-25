use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum SRVType {
    #[default]
    #[serde(rename = "testbuggy")]
    Scarab,

    #[serde(rename = "combat_multicrew_srv_01")]
    Scorpion,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for SRVType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SRVType::Scarab => "Scarab",
                SRVType::Scorpion => "Scorpion",

                #[cfg(feature = "allow-unknown")]
                SRVType::Unknown(unknown) => return write!(f, "Unknown SRV type: {}", unknown),
            }
        )
    }
}
