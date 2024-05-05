use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum FighterLoadout {
    #[serde(rename = "zero")]
    Zero,

    #[serde(rename = "starter")]
    Starter,
}
