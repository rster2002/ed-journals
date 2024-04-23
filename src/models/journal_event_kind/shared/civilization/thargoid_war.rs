use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ThargoidWar {
    pub current_state: ThargoidWarState,
    pub next_state_success: ThargoidWarState,
    pub next_state_failure: ThargoidWarState,
    pub success_state_reached: bool,
    pub war_progress: f32,
    pub remaining_ports: u8,

    // TODO parse this to a special struct
    pub estimated_remaining_time: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum ThargoidWarState {
    #[serde(rename = "Thargoid_Stronghold")]
    ThargoidStronghold,

    #[serde(rename = "")]
    Unspecified,

    #[cfg(not(feature = "strict"))]
    #[serde(untagged)]
    Unknown(String),
}
