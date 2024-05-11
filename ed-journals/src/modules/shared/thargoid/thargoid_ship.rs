use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum ThargoidShip {
    #[serde(rename = "scout_hq")]
    Scout,

    #[serde(rename = "glaive")]
    Glaive,
}
