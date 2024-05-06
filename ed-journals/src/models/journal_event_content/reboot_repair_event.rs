use serde::Deserialize;
use crate::journal_event_content::shared::ship::ship_slot::ShipSlot;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RebootRepairEvent {
    pub modules: Vec<ShipSlot>,
}
