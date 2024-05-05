use std::fmt::{Display, Formatter};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Superpower {
    Independent,
    Federation,
    Empire,
    Alliance,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for Superpower {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Superpower::Independent => "Independent",
            Superpower::Federation => "Federation",
            Superpower::Empire => "Empire",
            Superpower::Alliance => "Alliance",

            #[cfg(not(feature = "strict"))]
            Superpower::Unknown(unknown) => return write!(f, "Unknown superpower: {}", unknown),
        })
    }
}
