use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum EmpireRank {
    None,
    Outsider,
    Serf,
    Master,
    Squire,
    Knight,
    Lord,
    Baron,
    Viscount,
    Count,
    Earl,
    Marquis,
    Duke,
    Prince,
    King,
}
