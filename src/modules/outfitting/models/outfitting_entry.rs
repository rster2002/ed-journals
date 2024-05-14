use serde::Deserialize;
use crate::modules::models::ship::ship_module::ShipModule;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OutfittingEntry {
    #[serde(rename = "id")]
    pub id: u64,
    pub name: ShipModule,
    pub buy_price: u64,
}
