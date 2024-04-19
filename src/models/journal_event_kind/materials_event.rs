use serde::Deserialize;

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
    pub name: String,

    #[serde(rename = "Count")]
    pub count: u16,
}
