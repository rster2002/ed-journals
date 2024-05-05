use crate::models::journal_event_content::shared::materials::material::Material;
use crate::models::journal_event_content::shared::materials::material_category::MaterialCategory;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTradeEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub trader_type: MaterialCategory,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTradeEventExchange {
    pub material: Material,

    #[serde(rename = "Material_Localised")]
    pub material_localized: Option<String>,

    pub category: MaterialCategory,
    pub quantity: u16,
}
