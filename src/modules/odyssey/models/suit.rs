use std::num::ParseIntError;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::from_str_deserialize_impl;
use crate::modules::odyssey::{SuitType, SuitTypeError};

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct Suit {
    pub class: u8,
    pub kind: SuitType,
}

#[derive(Debug, Error)]
pub enum SuitError {
    #[error("Failed to parse suit: '{0}'")]
    FailedToParse(String),

    #[error("Failed to parse suit class: {0}")]
    FailedToParseClass(#[from] ParseIntError),

    #[error(transparent)]
    SuitTypeError(#[from] SuitTypeError),
}

lazy_static! {
    static ref SUIT_REGEX: Regex = Regex::new(r#"^(\w+?)(_[cC]lass(\d))?$"#).unwrap();
}

impl FromStr for Suit {
    type Err = SuitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(captures) = SUIT_REGEX.captures(s) else {
            return Err(SuitError::FailedToParse(s.to_string()));
        };

        Ok(Suit {
            class: match captures.get(3) {
                Some(class) => class.as_str().parse()?,
                None => 0,
            },
            kind: captures
                .get(1)
                .expect("Should have been captured already")
                .as_str()
                .parse()?,
        })
    }
}

from_str_deserialize_impl!(Suit);

// pub enum Suit {
//     #[serde(alias = "flightsuit")]
//     FlightSuit,
//
//     #[serde(alias = "ExplorationSuit_Class1", alias = "explorationsuit_class1")]
//     ArtemisSuit,
//
//     DominatorSuit,
//
//     #[serde(alias = "UtilitySuit_Class1", alias = "utilitysuit_class1")]
//     MaverickSuit,
//

// }
//
// impl Display for Suit {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Suit::FlightSuit => "Flight Suit",
//                 Suit::ArtemisSuit => "Artemis Suit",
//                 Suit::DominatorSuit => "Dominator Suit",
//                 Suit::MaverickSuit => "Maverick Suit",
//
//                 #[cfg(not(feature = "strict"))]
//                 Suit::Unknown(unknown) => return write!(f, "Unknown suit: {}", unknown),
//             }
//         )
//     }
// }
