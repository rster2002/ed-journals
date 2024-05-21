use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, ItemType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TransferMicroResourcesEvent {
    pub transfers: Vec<TransferMicroResourcesEventTransfer>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TransferMicroResourcesEventTransfer {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub category: ItemType,
    pub count: u8,
    pub direction: TransferMicroResourcesEventTransferDirection,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransferMicroResourcesEventTransferDirection {
    ToBackpack,
    // TODO the other way -_-
}
