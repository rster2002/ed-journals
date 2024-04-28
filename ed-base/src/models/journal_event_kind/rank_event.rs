use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RankEvent {
    pub combat: u8,
    pub trade: u8,
    pub explore: u8,
    pub empire: u8,
    pub federation: u8,

    #[serde(rename = "CQC")]
    pub cqc: u8,
}
