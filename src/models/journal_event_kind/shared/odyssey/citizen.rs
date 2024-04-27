use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum Citizen {
    #[serde(rename = "citizensuitai_scientific")]
    Researcher,

    // TODO needs a better name
    #[serde(rename = "assaultsuitai_class1")]
    AssaultSuit,
}
