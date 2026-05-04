//! Fired as part of the startup process of the game.

use serde::{Deserialize, Serialize};

/// Fired as part of the startup process of the game.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommanderEvent {
    /// The Frontier id associated with the player.
    #[serde(rename = "FID")]
    pub fid: String,

    /// The commander name of the player.
    #[serde(rename = "Name")]
    pub name: String,
}
