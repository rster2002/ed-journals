use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipModule;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct OutfittingEntry {
    #[serde(rename = "id")]
    pub id: u64,
    pub name: ShipModule,
    pub buy_price: u64,
}
