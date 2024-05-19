use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
}
