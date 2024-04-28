use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardNewEvent {
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: String,

    #[serde(rename = "NewShipID")]
    pub new_ship_id: u8,
}
