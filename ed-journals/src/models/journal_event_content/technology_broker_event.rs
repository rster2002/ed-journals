use crate::models::journal_event_content::shared::materials::material::Material;
use crate::models::journal_event_content::shared::materials::material_category::MaterialCategory;
use crate::models::journal_event_content::shared::ship::ship_module::ShipModule;
use crate::models::journal_event_content::shared::trading::commodity::Commodity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEvent {
    pub broker_type: TechnologyBrokerEventBrokerType,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub items_unlocked: Vec<TechnologyBrokerEventUnlockedItem>,
    pub commodities: Vec<TechnologyBrokerEventCommodity>,
    pub materials: Vec<TechnologyBrokerEventMaterial>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum TechnologyBrokerEventBrokerType {
    #[serde(rename = "guardian")]
    Guardian,

    #[serde(rename = "rescue")]
    Rescue,

    #[serde(rename = "human")]
    Human,

    #[serde(rename = "sirius")]
    Sirius,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEventUnlockedItem {
    pub name: ShipModule,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEventCommodity {
    pub name: Commodity,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u16,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEventMaterial {
    pub name: Material,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u16,
    pub category: MaterialCategory,
}
