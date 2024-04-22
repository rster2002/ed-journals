use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Government {
    #[serde(rename = "$government_Corporate;")]
    Corporate,

    #[serde(rename = "$government_Confederacy;")]
    Confederacy,

    #[serde(rename = "$government_Dictatorship;")]
    Dictatorship,

    #[serde(rename = "$government_Patronage;")]
    Patronage,

    #[serde(rename = "$government_None;")]
    None,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
