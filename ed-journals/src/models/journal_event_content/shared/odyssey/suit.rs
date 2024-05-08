use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Suit {
    #[serde(
        alias = "flightsuit",
    )]
    FlightSuit,

    #[serde(
        alias = "ExplorationSuit_Class1",
        alias = "explorationsuit_class1",
    )]
    ArtemisSuit,

    DominatorSuit,

    #[serde(
        alias = "UtilitySuit_Class1",
        alias = "utilitysuit_class1",
    )]
    MaverickSuit,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Suit::FlightSuit => "Flight Suit",
            Suit::ArtemisSuit => "Artemis Suit",
            Suit::DominatorSuit => "Dominator Suit",
            Suit::MaverickSuit => "Maverick Suit",

            #[cfg(not(feature = "strict"))]
            Suit::Unknown(unknown) => return write!(f, "Unknown suit: {}", unknown),
        })
    }
}
