use crate::modules::models::materials::material::Material;
use crate::modules::models::materials::material_category::MaterialCategory;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScientificResearchEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub name: Material,
    pub category: MaterialCategory,
    pub count: u8,
}
