use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ThargoidShip {
    #[serde(rename = "scout_hq")]
    Scout,

    #[serde(rename = "glaive")]
    Glaive,
}
