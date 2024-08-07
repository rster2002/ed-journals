use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PayFinesEvent {
    pub amount: u64,
    pub broker_percentage: Option<f32>,
    pub all_fines: bool,
    pub faction: Option<String>,

    #[serde(rename = "Faction_Localised")]
    pub faction_localized: Option<String>,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}
