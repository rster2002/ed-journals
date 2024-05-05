use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum FactionHappiness {
    #[serde(rename = "$Faction_HappinessBand1;")]
    Elated,

    #[serde(rename = "$Faction_HappinessBand2;")]
    Happy,

    #[serde(rename = "$Faction_HappinessBand3;")]
    Content,

    // TODO check what this one should actually be
    #[serde(rename = "")]
    Unspecified,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
