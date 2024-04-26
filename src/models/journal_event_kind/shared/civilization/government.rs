use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Government {
    Communism,

    #[serde(alias = "$government_Cooperative;")]
    Cooperative,

    #[serde(alias = "$government_Corporate;")]
    Corporate,

    #[serde(alias = "$government_Confederacy;")]
    Confederacy,

    #[serde(alias = "$government_Democracy;")]
    Democracy,

    #[serde(alias = "$government_Dictatorship;")]
    Dictatorship,

    #[serde(alias = "$government_Patronage;")]
    Patronage,

    #[serde(alias = "$government_Carrier;")]
    PrivateOwnership,

    #[serde(alias = "$government_Theocracy;")]
    Theocracy,

    #[serde(alias = "$government_Anarchy;")]
    Anarchy,

    #[serde(alias = "$government_Engineer;")]
    Engineer,

    #[serde(alias = "$government_Feudal;")]
    Feudal,

    #[serde(alias = "$government_PrisonColony;")]
    PrisonColony,

    #[serde(alias = "$government_None;")]
    None,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
