use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum SRVType {
    #[serde(rename = "testbuggy")]
    Scarab,
}
