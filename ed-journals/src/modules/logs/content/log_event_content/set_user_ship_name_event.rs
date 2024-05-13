use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SetUserShipNameEvent {
    // TODO replace with enum
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,

    pub user_ship_name: String,
    pub user_ship_id: String,
}
