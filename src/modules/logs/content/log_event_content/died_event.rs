use serde::{Deserialize, Serialize};

use crate::modules::commander::CombatRank;
use crate::modules::odyssey::Citizen;
use crate::modules::ship::ShipType;
use crate::modules::thargoid::ThargoidShip;

/// Fired when the player dies. Depending on the way the player died the event contains different
/// information.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum DiedEvent {
    /// The default value that does not contain any information related to the way the player died.
    None(DiedEventNone),

    /// When the player is killed by someone. This can be either be players or NPCs like Thargoids.
    IndividualKill(DiedEventIndividualKill),

    /// When the player is killed by a wing of other commanders.
    WingKill(DiedEventWingKill),
}

/// This should not contain any fields and is just here to make [serde] happy.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventNone {}

/// When the player is killed by someone. This can be either be players or NPCs like Thargoids.
/// This is also fired when killed by an on-foot combattant.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventIndividualKill {
    /// The name of the player's killer. This might be None when killed by for example a Thargoid.
    pub killer_name: Option<String>,

    /// The localized name of the player's killer, if any.
    #[serde(rename = "KillerName_Localised")]
    pub killer_name_localized: Option<String>,

    /// The ship of the killer. This could be a [ShipType], a [ThargoidShip] but importantly also a
    /// [Citizen] (yes even though it's called a killer SHIP)
    pub killer_ship: DiedEventKillerShip,

    /// The rank of the killer.
    pub killer_rank: CombatRank,
}

/// Event for when killed by a wing.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventWingKill {
    /// List of participating killers.
    pub killers: Vec<DiedEventWingKiller>,
}

/// A killer from a wing.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DiedEventWingKiller {
    /// The name of the commander that was part of the wing that killed the player.
    pub name: String,

    /// The ship the wing member.
    pub ship: ShipType,

    /// The rank of the killer.
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
