use serde::Deserialize;
use crate::models::journal_event_kind::shared::materials::material::Material;

#[derive(Debug, Deserialize)]
pub struct MaterialsEvent {
    #[serde(rename = "Raw")]
    pub raw: Vec<MaterialEventEntry>,

    #[serde(rename = "Encoded")]
    pub encoded: Vec<MaterialEventEntry>,

    #[serde(rename = "Manufactured")]
    pub manufactured: Vec<MaterialEventEntry>,
}

#[derive(Debug, Deserialize)]
pub struct MaterialEventEntry {
    #[serde(rename = "Name")]
    pub name: Material,

    #[serde(rename = "Count")]
    pub count: u16,
}
