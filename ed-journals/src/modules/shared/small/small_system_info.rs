use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SmallSystemInfo {
    pub system_address: u64,
    pub star_name: String,
}
