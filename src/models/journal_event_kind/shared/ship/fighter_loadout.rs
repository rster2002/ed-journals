use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FighterLoadout {
    #[serde(rename = "zero")]
    Zero,

    #[serde(rename = "starter")]
    Starter,
}
