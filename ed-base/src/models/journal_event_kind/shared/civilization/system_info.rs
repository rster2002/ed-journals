use crate::models::journal_event_kind::shared::civilization::conflict::Conflict;
use crate::models::journal_event_kind::shared::civilization::economy::Economy;
use crate::models::journal_event_kind::shared::civilization::faction::Faction;
use crate::models::journal_event_kind::shared::civilization::government::Government;
use crate::models::journal_event_kind::shared::civilization::superpower::Superpower;
use crate::models::journal_event_kind::shared::civilization::system_security::SystemSecurity;
use crate::models::journal_event_kind::shared::civilization::thargoid_war::ThargoidWar;
use crate::models::journal_event_kind::shared::galaxy::body_type::BodyType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfo {
    pub star_system: String,
    pub system_address: u64,
    pub star_pos: [f32; 3],
    pub system_alliance: Option<Superpower>,
    pub system_economy: Economy,

    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localized: String,

    pub system_second_economy: Economy,

    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localized: String,

    pub system_government: Government,

    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localized: String,

    pub system_security: SystemSecurity,

    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localized: String,
    pub population: u64,
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u8,
    pub body_type: BodyType,

    #[serde(default)]
    pub factions: Vec<Faction>,

    #[serde(default)]
    pub conflicts: Vec<Conflict>,
    pub thargoid_war: Option<ThargoidWar>,
    // TODO include powerplay
}
