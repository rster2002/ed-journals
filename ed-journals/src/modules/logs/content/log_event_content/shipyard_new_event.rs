use serde::{Serialize, Deserialize};

use crate::modules::shared::ship::ship_type::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardNewEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    #[serde(rename = "NewShipID")]
    pub new_ship_id: u8,
}
