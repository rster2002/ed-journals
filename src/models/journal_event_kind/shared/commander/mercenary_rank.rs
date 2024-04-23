use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum MercenaryRank {
    Defenceless,
    MostlyDefenceless,
    Rookie,
    Soldier,
    Gunslinger,
    Warrior,
    Gladiator,
    Deadeye,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,
}
