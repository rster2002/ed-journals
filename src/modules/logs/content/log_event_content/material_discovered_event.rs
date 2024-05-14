use serde::{Serialize, Deserialize};

use crate::modules::shared::materials::material::Material;
use crate::modules::shared::materials::material_category::MaterialCategory;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscoveredEvent {
    pub category: MaterialCategory,
    pub name: Material,
    pub discovery_number: u16,
}
