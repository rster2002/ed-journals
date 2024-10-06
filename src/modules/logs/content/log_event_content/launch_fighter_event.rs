use serde::{Deserialize, Serialize};

use crate::modules::ship::FighterLoadout;

/// Fired when launching a ship launched fighter.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LaunchFighterEvent {
    /// The used layout for the ship launched fighter.
    pub loadout: FighterLoadout,

    /// The id of the fighter.
    #[serde(rename = "ID")]
    pub id: u8,

    /// Whether the fighter is controlled by the current player.
    pub player_controlled: bool,
}
