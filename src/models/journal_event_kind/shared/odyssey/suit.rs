use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Suit {
    #[serde(rename = "ExplorationSuit_Class1")]
    ArtemisSuit,

    #[serde(rename = "utilitysuit_class1")]
    MaverickSuit,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
