use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(feature = "allow-unknown"), non_exhaustive)]
pub enum ThargoidShip {
    #[serde(rename = "scout_hq")]
    Scout,

    #[serde(rename = "glaive")]
    Glaive,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}
