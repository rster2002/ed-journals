use crate::models::journal_event_content::shared::odyssey::item::Item;
use crate::models::journal_event_content::shared::trading::commodity::Commodity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum SharedCommodity {
    ShipCommodity(Commodity),
    OdysseyItem(Item),
}
