use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FactionState {
    War,
    Election,
    Expansion,
    PublicHoliday,
    InfrastructureFailure,
    Boom,
    Terrorism,
    CivilWar,
    PirateAttack,
    Drought,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
