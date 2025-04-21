use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::backpack::models::backpack_entry::BackpackEntry;

/// Current state of the player's backpack.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Backpack {
    /// The timestamp of the lastest update.
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    /// The type of event, which is always "Backpack".
    #[serde(rename = "event")]
    pub event: String,

    /// List of items that the player currently has in their backpack that fall into the 'item'
    /// category.
    pub items: Vec<BackpackEntry>,

    /// List of items that the player currently has in their backpack that fall into the
    /// 'components' category.
    pub components: Vec<BackpackEntry>,

    /// List of items that the player currently has in their backpack that fall into the
    /// 'consumables' category.
    pub consumables: Vec<BackpackEntry>,

    /// List of items that the player currently has in their backpack that fall into the 'data'
    /// category.
    pub data: Vec<BackpackEntry>,
}
