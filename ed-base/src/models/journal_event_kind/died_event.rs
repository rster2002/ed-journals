use crate::models::journal_event_kind::shared::commander::combat_rank::CombatRank;
use crate::models::journal_event_kind::shared::odyssey::citizen::Citizen;
use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use crate::models::journal_event_kind::shared::thargoid::thargoid_ship::ThargoidShip;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase", untagged)]
pub enum DiedEvent {
    None(DiedEventNone),
    IndividualKill(DiedEventIndividualKill),
    WingKill(DiedEventWingKill),
}

/// This should not contain any fields and is just here to make [serde] happy.
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventNone {}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventIndividualKill {
    pub killer_name: Option<String>,

    #[serde(rename = "KillerName_Localised")]
    pub killer_name_localized: Option<String>,
    pub killer_ship: DiedEventKillerShip,
    pub killer_rank: CombatRank,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventWingKill {
    pub killers: Vec<DiedEventWingKiller>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventWingKiller {
    pub name: String,
    pub ship: ShipType,
    pub rank: CombatRank,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
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
