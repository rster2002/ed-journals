use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ThargoidShip {
    #[serde(rename = "scout_hq")]
    Scout,

    #[serde(rename = "glaive")]
    Glaive,
}
