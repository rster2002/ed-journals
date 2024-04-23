use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ExobiologyRank {
    Directionless,
    MostlyDirectionless,
    Compiler,
    Collector,
    Cataloguer,
    Taxonomist,
    Ecologist,
    Geneticist,
    Elite,
    EliteI,
    EliteII,
    EliteIII,
    EliteIV,
    EliteV,
}
