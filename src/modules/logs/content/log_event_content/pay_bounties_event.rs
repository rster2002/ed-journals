use serde::{Deserialize, Serialize};

/// Fired when the player has paid of their bounties.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PayBountiesEvent {
    /// The number of credits paid.
    pub amount: u64,

    /// The percentage the broker took.
    pub broker_percentage: Option<f32>,

    /// Whether all fines have been paid in one go.
    pub all_fines: bool,

    /// The faction the bounties were paid at.
    pub faction: Option<String>,

    /// The localized name of the faction the bounties were paid at.
    #[serde(rename = "Faction_Localised")]
    pub faction_localized: Option<String>,

    /// The id current active ship.
    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}
