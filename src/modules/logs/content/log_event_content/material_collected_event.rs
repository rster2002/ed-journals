use crate::materials::{Material, MaterialCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MaterialCollectedEvent {
    pub name: Material,
    pub category: MaterialCategory,
    pub count: u16,
}
