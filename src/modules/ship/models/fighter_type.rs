use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum FighterType {
    #[serde(rename = "independent_fighter")]
    Taipan,

    #[serde(rename = "empire_fighter")]
    GU97,

    #[serde(rename = "federation_fighter")]
    F63Condor,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}
