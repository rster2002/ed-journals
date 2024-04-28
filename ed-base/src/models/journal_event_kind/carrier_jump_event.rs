use serde::Deserialize;
use crate::models::journal_event_kind::shared::civilization::economy::Economy;
use crate::models::journal_event_kind::shared::civilization::faction::Faction;
use crate::models::journal_event_kind::shared::civilization::government::Government;
use crate::models::journal_event_kind::shared::civilization::superpower::Superpower;
use crate::models::journal_event_kind::shared::civilization::system_info::SystemInfo;
use crate::models::journal_event_kind::shared::civilization::system_security::SystemSecurity;
use crate::models::journal_event_kind::shared::galaxy::body_type::BodyType;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CarrierJumpEvent {
    pub docked: bool,

    #[serde(default)]
    pub on_foot: bool,

    #[serde(flatten)]
    pub system_info: SystemInfo,
}
