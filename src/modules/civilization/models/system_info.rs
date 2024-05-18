use serde::{Serialize, Deserialize};

use crate::modules::models::civilization::conflict::Conflict;
use crate::modules::models::civilization::economy::Economy;
use crate::modules::models::civilization::faction::Faction;
use crate::modules::models::civilization::government::Government;
use crate::modules::models::civilization::superpower::Superpower;
use crate::modules::models::civilization::system_security::SystemSecurity;
use crate::modules::models::civilization::thargoid_war::ThargoidWar;
use crate::modules::models::galaxy::body_type::BodyType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfo {
    /// The name of the star system that is displayed to the player.
    pub star_system: String,

    /// The unique 'id' of the system.s
    pub system_address: u64,

    /// The position of the system in the galaxy. The origin is the Sol system.
    pub star_pos: [f32; 3],

    /// To which superpower the system is allied, if any.
    pub system_alliance: Option<Superpower>,

    /// The economy of the system. Unpopulated systems use [Economy::None] as value.
    pub system_economy: Economy,

    /// Localized string of the [system_economy] field. Prefer using the [Display] trait on the
    /// [Economy] enum.
    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localized: Option<String>,

    /// Second economy of the system, if any.
    pub system_second_economy: Economy,

    /// Localized string of the [system_second_economy] field.
    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localized: Option<String>,

    pub system_government: Government,

    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localized: Option<String>,

    pub system_security: SystemSecurity,

    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localized: Option<String>,
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
