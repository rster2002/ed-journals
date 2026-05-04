use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ThargoidWar {
    pub current_state: ThargoidWarState,
    pub next_state_success: ThargoidWarState,
    pub next_state_failure: ThargoidWarState,
    pub success_state_reached: bool,
    pub war_progress: f32,
    pub remaining_ports: u8,

    // TODO parse this to a special struct
    // TODO check when this is [None]
    pub estimated_remaining_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub enum ThargoidWarState {
    #[serde(rename = "Thargoid_Stronghold")]
    Stronghold,

    #[serde(rename = "Thargoid_Probing")]
    Probing,

    #[serde(rename = "Thargoid_Controlled")]
    Controlled,

    #[serde(rename = "Thargoid_Recovery")]
    Recovery,

    #[serde(rename = "Thargoid_Harvest")]
    Harvest,

    #[serde(rename = "Unknown")]
    UnknownState,

    #[serde(rename = "")]
    Unspecified,

    #[cfg(feature = "allow-unknown")]
    #[cfg_attr(docsrs, doc(cfg(feature = "allow-unknown")))]
    #[serde(untagged)]
    Unknown(String),
}

impl Display for ThargoidWarState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ThargoidWarState::Stronghold => "Stronghold",
                ThargoidWarState::Probing => "Probing",
                ThargoidWarState::Controlled => "Controlled",
                ThargoidWarState::Recovery => "Recovery",
                ThargoidWarState::Harvest => "Harvest",
                ThargoidWarState::UnknownState => "Unknown",
                ThargoidWarState::Unspecified => "Unspecified",

                #[cfg(feature = "allow-unknown")]
                ThargoidWarState::Unknown(unknown) =>
                    return write!(f, "Unknown thargoid war state: {unknown}"),
            }
        )
    }
}
