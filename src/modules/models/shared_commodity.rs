use serde::{Serialize, Deserialize};

use crate::modules::models::odyssey::item::Item;
use crate::modules::models::trading::commodity::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SharedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),
}
