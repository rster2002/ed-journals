use serde::Deserialize;
use crate::models::ship::ship_type::ShipType;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardEntry {
    #[serde(rename = "id")]
    pub id: u64,
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,
    pub ship_price: u64,
}
