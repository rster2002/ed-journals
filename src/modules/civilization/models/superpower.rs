use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Superpower {
    Alliance,
    Empire,
    Federation,
    Guardian,
    Independent,
    PilotsFederation,
    PlayerPilots,
    Thargoid,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for Superpower {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Superpower::Alliance => "Alliance",
                Superpower::Empire => "Empire",
                Superpower::Federation => "Federation",
                Superpower::Guardian => "Guardian",
                Superpower::Independent => "Independent",
                Superpower::PilotsFederation => "Pilots Federation",
                Superpower::PlayerPilots => "Player Pilots",
                Superpower::Thargoid => "Thargoid",

                #[cfg(not(feature = "strict"))]
                Superpower::Unknown(unknown) =>
                    return write!(f, "Unknown superpower: {}", unknown),
            }
        )
    }
}
