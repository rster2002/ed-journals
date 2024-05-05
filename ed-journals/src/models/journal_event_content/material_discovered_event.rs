use crate::models::journal_event_content::shared::materials::material::Material;
use crate::models::journal_event_content::shared::materials::material_category::MaterialCategory;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscoveredEvent {
    pub category: MaterialCategory,
    pub name: Material,
    pub discovery_number: u16,
}
