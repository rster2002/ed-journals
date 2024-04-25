use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Government {
    #[serde(rename = "$government_Cooperative;")]
    Cooperative,

    #[serde(rename = "$government_Corporate;")]
    Corporate,

    #[serde(rename = "$government_Confederacy;")]
    Confederacy,

    #[serde(rename = "$government_Democracy;")]
    Democracy,

    #[serde(rename = "$government_Dictatorship;")]
    Dictatorship,

    #[serde(rename = "$government_Patronage;")]
    Patronage,

    #[serde(rename = "$government_Carrier;")]
    PrivateOwnership,

    #[serde(rename = "$government_Theocracy;")]
    Theocracy,

    #[serde(rename = "$government_Anarchy;")]
    Anarchy,

    #[serde(rename = "$government_Engineer;")]
    Engineer,

    #[serde(rename = "$government_Feudal;")]
    Feudal,

    #[serde(rename = "$government_PrisonColony;")]
    PrisonColony,

    #[serde(rename = "$government_None;")]
    None,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
