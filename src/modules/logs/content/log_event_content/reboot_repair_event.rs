use serde::{Serialize, Deserialize};
use crate::modules::ship::ShipSlot;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RebootRepairEvent {
    pub modules: Vec<ShipSlot>,
}
