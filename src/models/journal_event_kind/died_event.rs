use serde::Deserialize;
use crate::models::journal_event_kind::shared::commander::combat_rank::CombatRank;
use crate::models::journal_event_kind::shared::ship::ship_type::ShipType;
use crate::models::journal_event_kind::shared::thargoid::thargoid_ship::ThargoidShip;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase", untagged)]
pub enum DiedEvent {
    IndividualKill(DiedEventIndividualKill),
    WingKill(DiedEventWingKill),
}

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
    pub killers: Vec<DiedEventWingKiller>
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
    Human(ShipType),
    Thargoid(ThargoidShip),
}
