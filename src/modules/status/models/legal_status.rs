use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LegalStatus {
    Allied,
    Clean,
    None,
    Unknown,
    Lawless,
    Enemy,
    WantedEnemy,
    Hunter,
    IllegalCargo,
    Speeding,
    Wanted,
    Hostile,
    PassengerWanted,
    Warrant,
    Thargoid,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    UnknownString(String),
}
