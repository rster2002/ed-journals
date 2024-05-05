use serde::Deserialize;

use crate::models::journal_event_content::shared::odyssey::item::Item;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEvent {
    pub items: Vec<BackpackEventObject>,
    pub components: Vec<BackpackEventObject>,
    pub consumables: Vec<BackpackEventObject>,
    pub data: Vec<BackpackEventObject>,
}

// TODO this is the same as ship_locker_event
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BackpackEventObject {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "OwnerID")]
    pub owner_id: u64,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,
    pub count: u16,
}
