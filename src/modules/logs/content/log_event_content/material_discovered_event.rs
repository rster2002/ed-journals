use serde::{Serialize, Deserialize};
use crate::modules::materials::{Material, MaterialCategory};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialDiscoveredEvent {
    pub category: MaterialCategory,
    pub name: Material,
    pub discovery_number: u16,
}
