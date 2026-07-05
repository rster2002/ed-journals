use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ThargoidShip {
    #[serde(rename = "scout_hq")]
    Scout,

    #[serde(rename = "glaive")]
    Glaive,
}
