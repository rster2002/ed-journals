use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CombatRank {
    Harmless,
    MostlyHarmless,
    Novice,
    Competent,
    Expert,
    Master,
    Dangerous,
    Deadly,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,
}

#[derive(Debug, Error)]
pub enum CombatRankError {
    #[error("There is no combat rank")]
    UnknownCombatRank(u8),
}

impl TryFrom<u8> for CombatRank {

}
