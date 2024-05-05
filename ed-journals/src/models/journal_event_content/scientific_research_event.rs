use serde::Deserialize;
use crate::journal_event_content::shared::materials::material::Material;
use crate::journal_event_content::shared::materials::material_category::MaterialCategory;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScientificResearchEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub name: Material,
    pub category: MaterialCategory,
    pub count: u8,
}
