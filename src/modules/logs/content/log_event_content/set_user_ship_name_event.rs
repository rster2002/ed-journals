use serde::{Serialize, Deserialize};
use crate::modules::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SetUserShipNameEvent {
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    pub user_ship_name: String,
    pub user_ship_id: String,
}
