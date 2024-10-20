use serde::{Deserialize, Serialize};

/// Emitted when opening the outfitting menu. The contents of the outfitting menu are written to a
/// separate file called `Outfitting.json` in the journal directory.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OutfittingEvent {
    /// The market id the player is currently at.
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    /// The name of the station the player is currently docked at and opened the outfitting menu at.
    pub station_name: String,

    /// The name of the star system the player is currently in.
    pub star_system: String,
}
