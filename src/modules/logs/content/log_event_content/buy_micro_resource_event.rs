//! Fired when performing a trade with Odyssey bartenders.

use serde::{Deserialize, Serialize};

use crate::modules::odyssey::{Item, ItemType};

/// Fired when performing a trade with Odyssey bartenders.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum BuyMicroResourceEvent {
    /// When making a single trade.
    Single(BuyMicroResourceEventResource),

    /// When making multiple trades at once.
    Multiple(BuyMicroResourceEventMultiple),
}

/// When making a single trade.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyMicroResourceEventSingle {
    /// The item that was bought.
    pub name: Item,

    /// The localized name of the item that was bought.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The category of the item bought.
    pub category: ItemType,

    /// The number of items that were bought.
    pub count: u16,

    /// The number of credits that was paid.
    pub price: u64,

    /// The market id the purchase was made at.
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

/// When making multiple trades at once.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyMicroResourceEventMultiple {
    /// The total amount of resources traded.
    pub total_count: u16,

    /// List of resources that were bought.
    pub micro_resources: Vec<BuyMicroResourceEventResource>,

    /// The total number of credits that was paid.
    pub price: u64,

    /// The market id the purchase was made at.
    #[serde(rename = "MarketID")]
    pub market_id: u64,
}

/// Entry for an item the player has bought.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BuyMicroResourceEventResource {
    /// The item that was bought.
    pub name: Item,

    /// The localized name of the item that was bought.
    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,

    /// The category of the item bought.
    pub category: ItemType,

    /// The number of items that were bought.
    pub count: u16,
}
