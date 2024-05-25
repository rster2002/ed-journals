use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FighterType {
    #[serde(rename = "independent_fighter")]
    Taipan,

    #[serde(rename = "empire_fighter")]
    GU97,

    #[serde(rename = "federation_fighter")]
    F63Condor,
}
