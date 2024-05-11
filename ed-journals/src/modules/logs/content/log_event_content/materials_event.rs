use crate::modules::shared::materials::material::Material;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MaterialsEvent {
    #[serde(rename = "Raw")]
    pub raw: Vec<MaterialEventEntry>,

    #[serde(rename = "Encoded")]
    pub encoded: Vec<MaterialEventEntry>,

    #[serde(rename = "Manufactured")]
    pub manufactured: Vec<MaterialEventEntry>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct MaterialEventEntry {
    #[serde(rename = "Name")]
    pub name: Material,

    #[serde(rename = "Count")]
    pub count: u16,
}
