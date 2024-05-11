use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum Citizen {
    #[serde(rename = "citizensuitai_scientific")]
    Researcher,

    #[serde(
        rename = "assaultsuitai_class1",
        alias = "assaultsuitai_class2",
        alias = "assaultsuitai_class3"
    )]
    Commando,

    #[serde(
        rename = "lightassaultsuitai_class1",
        alias = "lightassaultsuitai_class2",
        alias = "lightassaultsuitai_class3"
    )]
    Scout,

    #[serde(rename = "closesuitai_class1", alias = "closesuitai_class3")]
    Striker,

    // TODO this is a guess
    #[serde(
        rename = "rangedsuitai_class1",
        alias = "rangedsuitai_class2",
        alias = "rangedsuitai_class3"
    )]
    Sniper,
}

impl Display for Citizen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Citizen::Researcher => "Researcher",
                Citizen::Commando => "Commando",
                Citizen::Scout => "Scout",
                Citizen::Striker => "Striker",
                Citizen::Sniper => "Sniper",
            }
        )
    }
}
