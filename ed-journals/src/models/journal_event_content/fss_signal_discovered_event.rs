use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FSSSignalDiscoveredEvent {
    pub system_address: u64,
    pub signal_name: String,

    #[serde(rename = "SignalName_Localised")]
    pub signal_name_localised: Option<String>,

    #[serde(default)]
    pub is_station: bool,
}
