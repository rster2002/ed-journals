use serde::Deserialize;
use crate::models::journal_event_kind::shared::materials::material::Material;
use crate::models::journal_event_kind::shared::materials::material_category::{MaterialCategory};

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTradeEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub trader_type: MaterialCategory,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MaterialTradeEventExchange {
    pub material: Material,

    #[serde(rename = "Material_Localised")]
    pub material_localized: String,

    pub category: MaterialCategory,
    pub quantity: u16,
}
