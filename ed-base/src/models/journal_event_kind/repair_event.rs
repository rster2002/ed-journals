use serde::Deserialize;
use crate::models::journal_event_kind::shared::ship::ship_module::ShipModule;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct RepairEvent {
    pub items: Vec<String>,
    pub cost: u64,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum RepairEventItem {
    Hull,
    Wear,
    Module(ShipModule),
}
