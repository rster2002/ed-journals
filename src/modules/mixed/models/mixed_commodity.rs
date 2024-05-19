use serde::{Serialize, Deserialize};
use crate::modules::odyssey::Item;
use crate::modules::trading::Commodity;

/// Special model for handling scenarios where the input might either be a 'ship' commodity or an
/// Odyssey commodity.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum MixedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),
}
