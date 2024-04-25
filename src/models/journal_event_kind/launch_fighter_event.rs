use serde::Deserialize;
use crate::models::journal_event_kind::restock_vehicle_event::RestockVehicleEventLoadout;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LaunchFighterEvent {
    pub loadout: LaunchFighterEventLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
    pub player_controlled: bool,
}

// TODO this seems to be the same as RestockVehicleEventLoadout
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LaunchFighterEventLoadout {
    #[serde(rename = "zero")]
    Zero,

    #[serde(rename = "starter")]
    Starter,
}
