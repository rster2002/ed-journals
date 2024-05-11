use crate::modules::shared::ship::ship_slot::ShipSlot;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RebootRepairEvent {
    pub modules: Vec<ShipSlot>,
}
