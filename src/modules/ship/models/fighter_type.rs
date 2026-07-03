use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum FighterType {
    #[serde(rename = "independent_fighter")]
    Taipan,

    #[serde(rename = "empire_fighter")]
    GU97,

    #[serde(rename = "federation_fighter")]
    F63Condor,

    #[serde(rename = "gdn_hybrid_fighter_v1")]
    GuardianTridentXG7,

    #[serde(rename = "gdn_hybrid_fighter_v2")]
    GuardianJavelinXG8,

    #[serde(rename = "gdn_hybrid_fighter_v3")]
    GuardianLanceXG9,

    #[serde(rename = "lander01")]
    Nomad,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for FighterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FighterType::Taipan => "Taipan",
                FighterType::GU97 => "GU97",
                FighterType::F63Condor => "F63 Condor",
                FighterType::GuardianTridentXG7 => "Guardian Trident XG7",
                FighterType::GuardianJavelinXG8 => "Guardian Javelin XG8",
                FighterType::GuardianLanceXG9 => "Guardian Lance XG9",
                FighterType::Nomad => "Nomad",

                #[cfg(feature = "allow-unknown")]
                #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
                FighterType::Unknown(unknown) => unknown,
            }
        )
    }
}
