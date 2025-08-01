//! Fired when the player buys ammo at a station.

use serde::{Deserialize, Serialize};

/// Fired when the player buys ammo at a station.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct BuyAmmoEvent {
    /// The amount of credits spent to fully restock all weapons.
    pub cost: u64,
}
