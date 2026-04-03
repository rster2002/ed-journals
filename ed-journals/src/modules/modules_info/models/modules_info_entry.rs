use serde::{Deserialize, Serialize};

use crate::modules::ship::{ShipModule, ShipSlot};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ModulesInfoEntry {
    pub slot: ShipSlot,
    pub item: ShipModule,
    pub power: f32,
    pub priority: Option<u8>,
}
