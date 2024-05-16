use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FighterLoadout {
    #[serde(rename = "zero")]
    Zero,

    #[serde(rename = "starter")]
    Starter,
}
