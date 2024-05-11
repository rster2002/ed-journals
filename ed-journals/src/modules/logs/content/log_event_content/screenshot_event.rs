use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ScreenshotEvent {
    pub filename: String,
    pub width: u16,
    pub height: u16,
    pub system: String,
    pub body: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub altitude: Option<f32>,
    pub heading: Option<f32>,
}
