use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ThargoidWar {
    pub current_state: ThargoidWarState,
    pub next_state_success: ThargoidWarState,
    pub next_state_failure: ThargoidWarState,
    pub success_state_reached: ThargoidWarState,
    pub war_progress: u8,
    pub remaining_ports: u8,
    pub estimated_remaining_time: u32,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum ThargoidWarState {
    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
