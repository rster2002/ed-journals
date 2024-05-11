use serde::Deserialize;

use crate::modules::shared::materials::material::Material;

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
