use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Suit {
    #[serde(rename = "ExplorationSuit_Class1", alias = "explorationsuit_class1")]
    ArtemisSuit,

    #[serde(rename = "UtilitySuit_Class1", alias = "utilitysuit_class1")]
    MaverickSuit,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
