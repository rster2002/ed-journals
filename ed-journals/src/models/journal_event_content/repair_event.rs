use crate::models::journal_event_content::shared::ship::ship_module::ShipModule;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct RepairEvent {
    pub items: Vec<String>,
    pub cost: u64,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum RepairEventItem {
    Hull,
    Wear,
    Module(ShipModule),
}
