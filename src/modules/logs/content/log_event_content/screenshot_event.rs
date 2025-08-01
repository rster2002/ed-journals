use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ScreenshotEvent {
    pub filename: String,
    pub width: u16,
    pub height: u16,
    pub system: Option<String>,
    pub body: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub altitude: Option<f32>,
    pub heading: Option<f32>,
}
