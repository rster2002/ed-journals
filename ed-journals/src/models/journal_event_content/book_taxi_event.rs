use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct BookTaxiEvent {
    pub cost: u64,
    pub destination_system: String,
    pub destination_location: String,

    #[serde(default)]
    pub retreat: bool,
}
