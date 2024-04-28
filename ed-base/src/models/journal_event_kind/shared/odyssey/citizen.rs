use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Citizen {
    #[serde(rename = "citizensuitai_scientific")]
    Researcher,

    #[serde(rename = "assaultsuitai_class1", alias = "assaultsuitai_class2")]
    Commando,

    #[serde(
        rename = "lightassaultsuitai_class1",
        alias = "lightassaultsuitai_class2"
    )]
    Scout,

    #[serde(rename = "closesuitai_class1")]
    Striker,

    // TODO this is a guess
    #[serde(rename = "rangedsuitai_class2")]
    Sniper,
}
