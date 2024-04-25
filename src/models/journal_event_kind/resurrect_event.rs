use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ResurrectEvent {
    pub option: ResurrectEventOption,
    pub cost: u64,
    pub bankrupt: bool,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ResurrectEventOption {
    #[serde(rename = "handin")]
    HandIn,

    #[serde(rename = "rebuy")]
    Rebuy,
}
