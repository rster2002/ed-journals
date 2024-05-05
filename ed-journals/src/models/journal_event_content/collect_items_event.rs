use crate::models::journal_event_content::shared::odyssey::item::Item;
use crate::models::journal_event_content::shared::odyssey::item_type::ItemType;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CollectItemsEvent {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    #[serde(rename = "Type")]
    pub kind: ItemType,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,
    pub count: u16,
    pub stolen: bool,
}