use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FactionHappiness {
    #[serde(rename = "$Faction_HappinessBand1;")]
    Elated,

    #[serde(rename = "$Faction_HappinessBand2;")]
    Happy,

    #[serde(rename = "$Faction_HappinessBand3;")]
    Discontent,

    #[serde(rename = "$Faction_HappinessBand4;")]
    Unhappy,

    #[serde(rename = "$Faction_HappinessBand5;")]
    Despondent,

    // TODO check what this one should actually be
    #[serde(rename = "")]
    Unspecified,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}
