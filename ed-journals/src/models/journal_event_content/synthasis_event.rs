use crate::models::journal_event_content::shared::materials::material::Material;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SynthesisEvent {
    // TODO replace with enum
    pub name: String,
    pub materials: Vec<SynthesisEventMaterial>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SynthesisEventMaterial {
    pub name: Material,
    pub count: u8,
}
