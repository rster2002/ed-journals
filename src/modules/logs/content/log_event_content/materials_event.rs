use crate::modules::models::materials::material::Material;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MaterialsEvent {
    #[serde(rename = "Raw")]
    pub raw: Vec<MaterialEventEntry>,

    #[serde(rename = "Encoded")]
    pub encoded: Vec<MaterialEventEntry>,

    #[serde(rename = "Manufactured")]
    pub manufactured: Vec<MaterialEventEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MaterialEventEntry {
    #[serde(rename = "Name")]
    pub name: Material,

    #[serde(rename = "Count")]
    pub count: u16,
}
