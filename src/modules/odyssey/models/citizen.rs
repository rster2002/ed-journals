use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

/// This is used for bounty targets and other events to specify on foot NPCs.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Citizen {
    #[serde(rename = "citizensuitai_scientific")]
    Researcher,

    #[serde(rename = "citizensuitai_industrial")]
    Technician,

    #[serde(
        rename = "assaultsuitai_class1",
        alias = "assaultsuitai_class2",
        alias = "assaultsuitai_class3"
    )]
    Commando,

    // TODO this is a guess
    #[serde(
        rename = "heavysuitai_class1",
        alias = "heavysuitai_class2",
        alias = "heavysuitai_class3"
    )]
    Enforcer,

    #[serde(
        rename = "lightassaultsuitai_class1",
        alias = "lightassaultsuitai_class2",
        alias = "lightassaultsuitai_class3"
    )]
    Scout,

    #[serde(
        rename = "closesuitai_class1",
        alias = "closesuitai_class2",
        alias = "closesuitai_class3"
    )]
    Striker,

    // TODO this is a guess
    #[serde(
        rename = "rangedsuitai_class1",
        alias = "rangedsuitai_class2",
        alias = "rangedsuitai_class3"
    )]
    Sniper,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for Citizen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Citizen::Researcher => "Researcher",
                Citizen::Technician => "Technician",
                Citizen::Commando => "Commando",
                Citizen::Enforcer => "Enforcer",
                Citizen::Scout => "Scout",
                Citizen::Striker => "Striker",
                Citizen::Sniper => "Sniper",

                #[cfg(feature = "allow-unknown")]
                Citizen::Unknown(unknown) => return write!(f, "Unknown citizen: {}", unknown),
            }
        )
    }
}
