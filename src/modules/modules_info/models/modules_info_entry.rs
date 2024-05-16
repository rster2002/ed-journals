use serde::Deserialize;
use crate::models::ship::ship_module::ShipModule;
use crate::models::ship::ship_slot::ShipSlot;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ModulesInfoEntry {
    pub slot: ShipSlot,
    pub item: ShipModule,
    pub power: f32,
    pub priority: Option<u8>,
}
