use serde::{Deserialize, Serialize};

use crate::modules::materials::{Material, MaterialCategory};
use crate::modules::ship::ShipModule;
use crate::modules::trading::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEvent {
    pub broker_type: TechnologyBrokerEventBrokerType,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub items_unlocked: Vec<TechnologyBrokerEventUnlockedItem>,
    pub commodities: Vec<TechnologyBrokerEventCommodity>,
    pub materials: Vec<TechnologyBrokerEventMaterial>,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum TechnologyBrokerEventBrokerType {
    #[serde(rename = "guardian")]
    Guardian,

    #[serde(rename = "rescue")]
    Rescue,

    #[serde(rename = "human")]
    Human,

    #[serde(rename = "sirius")]
    Sirius,

    #[serde(rename = "salvation")]
    Salvation,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEventUnlockedItem {
    pub name: ShipModule,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEventCommodity {
    pub name: Commodity,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct TechnologyBrokerEventMaterial {
    pub name: Material,

    #[serde(rename = "Name_Localised")]
    pub name_localized: Option<String>,
    pub count: u16,
    pub category: MaterialCategory,
}
