use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MultiSellExplorationDataEvent {
    pub base_value: u64,
    pub bonus: u64,
    pub total_earnings: u64,
    pub discovered: Vec<MultiSellExplorationDataEventDiscovery>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct MultiSellExplorationDataEventDiscovery {
    pub system_name: String,

    #[serde(rename = "NumBodies")]
    pub number_of_bodies: u8,
}
