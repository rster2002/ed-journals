use serde::Deserialize;

/// Fired when the whole multi-crew session has ended.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct EndCrewSessionEvent {
    #[serde(default)]
    pub on_crime: bool,

    #[serde(default)]
    pub telepresence: bool,
}
