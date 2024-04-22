use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum FactionState {
    War,
    Election,
    Expansion,
    PublicHoliday,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
