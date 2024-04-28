use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct FSSAllBodiesFoundEvent {
    pub system_name: String,
    pub system_address: u64,
    pub count: u8,
}
