use serde::Deserialize;

use crate::modules::shared::odyssey::item::Item;
use crate::modules::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SharedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),
}
