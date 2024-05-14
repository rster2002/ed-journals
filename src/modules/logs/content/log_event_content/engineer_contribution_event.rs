use serde::{Serialize, Deserialize};

use crate::modules::shared::materials::material::Material;
use crate::modules::shared::trading::commodity::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEvent {
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "Type", flatten)]
    pub kind: EngineerContributionEventType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", tag = "Type")]
pub enum EngineerContributionEventType {
    Commodity(EngineerContributionEventCommodityContribution),
    Materials(EngineerContributionEventMaterialContribution),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEventCommodityContribution {
    pub commodity: Commodity,
    pub quantity: u16,
    pub total_quantity: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEventMaterialContribution {
    pub material: Material,
    pub quantity: u16,
    pub total_quantity: u16,
}
