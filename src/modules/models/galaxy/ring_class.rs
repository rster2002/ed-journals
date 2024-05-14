use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RingClass {
    #[serde(rename = "eRingClass_Rocky")]
    Rocky,

    #[serde(rename = "eRingClass_Metalic")]
    Metallic,

    #[serde(rename = "eRingClass_MetalRich")]
    MetalRich,

    #[serde(rename = "eRingClass_Icy")]
    Icy,
}
