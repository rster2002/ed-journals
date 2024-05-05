use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum FactionState {
    Blight,
    Boom,
    Bust,
    CivilUnrest,
    CivilWar,
    Drought,
    Election,
    Expansion,
    Famine,
    InfrastructureFailure,
    NaturalDisaster,
    PirateAttack,
    PublicHoliday,
    Terrorism,
    War,
    CivilLiberty,
    Outbreak,
    Retreat,
    Lockdown,
    Investment,
    None,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
