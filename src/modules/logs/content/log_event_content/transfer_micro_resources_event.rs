use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, ItemCategory};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct TransferMicroResourcesEvent {
    pub transfers: Vec<TransferMicroResourcesEventTransfer>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct TransferMicroResourcesEventTransfer {
    pub name: Item,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub category: ItemCategory,
    pub count: u16,
    pub direction: TransferMicroResourcesEventTransferDirection,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TransferMicroResourcesEventTransferDirection {
    ToBackpack,
    // TODO the other way -_-
}
