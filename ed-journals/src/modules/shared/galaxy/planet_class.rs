use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum PlanetClass {
    #[serde(rename = "Metal rich body")]
    MetalRichBody,

    #[serde(rename = "High metal content body")]
    HighMetalContentBody,

    #[serde(rename = "Rocky body")]
    RockyBody,

    #[serde(rename = "Icy body")]
    IcyBody,

    #[serde(rename = "Rocky ice body")]
    RockyIceBody,

    #[serde(rename = "Earthlike body")]
    EarthlikeBody,

    #[serde(rename = "Water world")]
    WaterWorld,

    #[serde(rename = "Ammonia world")]
    AmmoniaWorld,

    #[serde(rename = "Water giant")]
    WaterGiant,

    #[serde(rename = "Water giant with life")]
    WaterGiantWithLife,

    #[serde(rename = "Gas giant with water based life")]
    GasGiantWithWaterBasedLife,

    #[serde(rename = "Gas giant with ammonia based life")]
    GasGiantWithAmmoniaBasedLife,

    #[serde(rename = "Sudarsky class I gas giant")]
    SudarskyClassIGasGiant,

    #[serde(rename = "Sudarsky class II gas giant")]
    SudarskyClassIiGasGiant,

    #[serde(rename = "Sudarsky class III gas giant")]
    SudarskyClassIiiGasGiant,

    #[serde(rename = "Sudarsky class IV gas giant")]
    SudarskyClassIvGasGiant,

    #[serde(rename = "Sudarsky class V gas giant")]
    SudarskyClassVGasGiant,

    #[serde(rename = "Helium rich gas giant")]
    HeliumRichGasGiant,

    #[serde(rename = "Helium gas giant")]
    HeliumGasGiant,
}
