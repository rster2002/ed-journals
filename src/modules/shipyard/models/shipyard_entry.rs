use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardEntry {
    #[serde(rename = "id")]
    pub id: u64,
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,
    pub ship_price: u64,
}
