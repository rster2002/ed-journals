use serde::Deserialize;

/// Emitted when opening the outfitting menu. The contents of the outfitting menu are written to a
/// separate file called `Outfitting.json` in the journal directory.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OutfittingEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub star_system: String,
}
