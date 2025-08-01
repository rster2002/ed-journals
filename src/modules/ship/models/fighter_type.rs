use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
}
