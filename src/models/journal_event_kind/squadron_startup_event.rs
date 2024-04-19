use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SquadronStartupEvent {
    pub squadron_name: String,
    pub current_rank: u8,
}
