//! Fired when the player clear their save, deleting all progress.

use serde::{Deserialize, Serialize};

/// Fired when the player clear their save, deleting all progress.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ClearSavedGameEvent {
    /// The name of the player.
    pub name: String,

    /// The Frontier id for the player.
    #[serde(rename = "FID")]
    pub fid: String,
}
