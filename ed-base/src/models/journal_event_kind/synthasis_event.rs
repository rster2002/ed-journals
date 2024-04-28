use serde::Deserialize;
use crate::models::journal_event_kind::shared::materials::material::Material;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SynthesisEvent {
    // TODO replace with enum
    pub name: String,
    pub materials: Vec<SynthesisEventMaterial>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SynthesisEventMaterial {
    pub name: Material,
    pub count: u8,
}
