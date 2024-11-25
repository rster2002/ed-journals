use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum FighterLoadout {
    #[serde(rename = "zero")]
    Zero,

    #[serde(rename = "one")]
    One,

    #[serde(rename = "two")]
    Two,

    #[serde(rename = "three")]
    Three,

    #[serde(rename = "four")]
    Four,

    #[serde(rename = "default")]
    Default,

    #[serde(rename = "starter")]
    Starter,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}
