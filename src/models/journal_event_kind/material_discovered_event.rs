use serde::Deserialize;
use crate::models::journal_event_kind::shared::materials::material::Material;
use crate::models::journal_event_kind::shared::materials::material_type::MaterialType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscoveredEvent {
    pub category: MaterialType,
    pub name: Material,
    pub discovery_number: u16,
}
