use serde::{Serialize, Deserialize};

use crate::modules::models::commander::combat_rank::CombatRank;
use crate::modules::models::odyssey::citizen::Citizen;
use crate::modules::models::ship::ship_type::ShipType;
use crate::modules::models::thargoid::thargoid_ship::ThargoidShip;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum DiedEvent {
    None(DiedEventNone),
    IndividualKill(DiedEventIndividualKill),
    WingKill(DiedEventWingKill),
}

/// This should not contain any fields and is just here to make [serde] happy.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventNone {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventIndividualKill {
    pub killer_name: Option<String>,

    #[serde(rename = "KillerName_Localised")]
    pub killer_name_localized: Option<String>,
    pub killer_ship: DiedEventKillerShip,
    pub killer_rank: CombatRank,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventWingKill {
    pub killers: Vec<DiedEventWingKiller>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventWingKiller {
    pub name: String,
    pub ship: ShipType,
    pub rank: CombatRank,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum DiedEventKillerShip {
    /// When killed by a human ship.
    Human(ShipType),

    /// When killed by a Thargoid vessel.
    Thargoid(ThargoidShip),

    // Ah yes, of course, the on foot combattant killer SHIP
    /// When killed by an on-foot combattant.
    Citizen(Citizen),
}
