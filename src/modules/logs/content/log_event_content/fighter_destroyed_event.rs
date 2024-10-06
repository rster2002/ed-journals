use serde::{Deserialize, Serialize};

/// Fired when a deployed ship launched fighter was destroyed.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FighterDestroyedEvent {
    /// The id of the destroyed ship launched fighter.
    #[serde(rename = "ID")]
    pub id: u8,
}
