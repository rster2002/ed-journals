use serde::Deserialize;
use crate::models::journal_event_kind::shared::odyssey::item::Item;
use crate::models::journal_event_kind::shared::trading::commodity::Commodity;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(untagged)]
pub enum SharedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),
}
