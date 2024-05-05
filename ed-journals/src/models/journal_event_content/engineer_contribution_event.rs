use crate::models::journal_event_content::shared::materials::material::Material;
use crate::models::journal_event_content::shared::trading::commodity::Commodity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEvent {
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "Type", flatten)]
    pub kind: EngineerContributionEventType,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", tag = "Type")]
pub enum EngineerContributionEventType {
    Commodity(EngineerContributionEventCommodityContribution),
    Materials(EngineerContributionEventMaterialContribution),
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEventCommodityContribution {
    pub commodity: Commodity,
    pub quantity: u16,
    pub total_quantity: u16,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EngineerContributionEventMaterialContribution {
    pub material: Material,
    pub quantity: u16,
    pub total_quantity: u16,
}
