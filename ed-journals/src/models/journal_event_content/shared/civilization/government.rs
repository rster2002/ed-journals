use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Government {
    #[serde(alias = "$government_Anarchy;")]
    Anarchy,

    #[serde(alias = "$government_Communism;")]
    Communism,

    #[serde(alias = "$government_Confederacy;")]
    Confederacy,

    #[serde(alias = "$government_Cooperative;")]
    Cooperative,

    #[serde(alias = "$government_Corporate;")]
    Corporate,

    #[serde(alias = "$government_Democracy;")]
    Democracy,

    #[serde(alias = "$government_Dictatorship;")]
    Dictatorship,

    #[serde(alias = "$government_Feudal;")]
    Feudal,

    #[serde(alias = "$government_Patronage;")]
    Patronage,

    #[serde(alias = "$government_PrisonColony;")]
    PrisonColony,

    #[serde(alias = "$government_Theocracy;")]
    Theocracy,

    #[serde(alias = "$government_Engineer;")]
    Engineer,

    #[serde(alias = "$government_None;")]
    None,

    #[serde(alias = "$government_Prison;")]
    Prison,

    #[serde(alias = "$government_Carrier;")]
    PrivateOwnership,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for Government {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Government::Anarchy => "Anarchy",
            Government::Communism => "Communism",
            Government::Confederacy => "Confederacy",
            Government::Cooperative => "Cooperative",
            Government::Corporate => "Corporate",
            Government::Democracy => "Democracy",
            Government::Dictatorship => "Dictatorship",
            Government::Engineer => "Engineer",
            Government::Feudal => "Feudal",
            Government::Patronage => "Patronage",
            Government::Prison => "Prison",
            Government::PrisonColony => "Prison Colony",
            Government::PrivateOwnership => "Private Ownership",
            Government::Theocracy => "Theocracy",

            Government::None => "None",

            #[cfg(not(feature = "strict"))]
            Government::Unknown(unknown) => return write!(f, "Unknown government: {}", unknown),
        })
    }
}
