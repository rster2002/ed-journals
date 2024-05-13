use serde::{Serialize, Deserialize};

use crate::modules::shared::odyssey::item::Item;
use crate::modules::shared::trading::commodity::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SharedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),
}
