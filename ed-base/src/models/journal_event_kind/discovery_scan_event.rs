use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DiscoveryScanEvent {
    pub system_address: u64,
    pub bodies: u8,
}
