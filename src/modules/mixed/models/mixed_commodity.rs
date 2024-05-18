use serde::{Serialize, Deserialize};

use crate::modules::models::odyssey::item::Item;
use crate::modules::models::trading::commodity::Commodity;

/// Special model for handling scenarios where the input might either be a 'ship' commodity or an
/// Odyssey commodity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MixedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),
}
