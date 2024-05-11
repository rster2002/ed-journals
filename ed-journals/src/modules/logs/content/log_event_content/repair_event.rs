use serde::Deserialize;

use crate::modules::shared::ship::ship_module::ShipModule;

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
