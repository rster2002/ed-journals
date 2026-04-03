use serde::{Deserialize, Serialize};

use crate::modules::civilization::{
    Conflict, Economy, Faction, Government, Superpower, SystemSecurity, ThargoidWar,
};
use crate::modules::galaxy::BodyType;

/// Shared model for information about a given location.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LocationInfo {
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

    /// The primary government in the system.
    pub system_government: Government,

    /// The localized name of the primary government in the system.
    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localized: Option<String>,

    /// The level of security presence in the system.
    pub system_security: SystemSecurity,

    /// The localized name of the security presence in the system.
    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localized: Option<String>,

    /// The number of citizens that live in the system.
    pub population: u64,

    /// The name of the body the location refers to.
    pub body: String,

    /// The id of the body the location refers to.
    #[serde(rename = "BodyID")]
    pub body_id: u8,

    /// The kind of body the location refers to.
    pub body_type: BodyType,

    /// A list of factions that are present in the system.
    #[serde(default)]
    pub factions: Vec<Faction>,

    /// A list of conflicts/wars currently active in the system.
    #[serde(default)]
    pub conflicts: Vec<Conflict>,

    /// Information about the state of the Thargoid war in the current system, if any.
    pub thargoid_war: Option<ThargoidWar>,
    // TODO include powerplay
}
