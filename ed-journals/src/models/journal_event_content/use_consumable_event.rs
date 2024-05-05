use crate::models::journal_event_content::shared::odyssey::item::Item;
use crate::models::journal_event_content::shared::odyssey::item_type::ItemType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct UseConsumableEvent {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "Type")]
    pub kind: ItemType,
}