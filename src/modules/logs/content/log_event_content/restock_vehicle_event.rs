use serde::{Deserialize, Serialize};

use crate::modules::ship::FighterLoadout;
use crate::ship::{FighterType, SRVType};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct RestockVehicleEvent {
    #[serde(rename = "Type")]
    pub kind: RestockVehicleEventType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: Option<String>,
    pub loadout: FighterLoadout,

    // TODO figure out when this is [None]
    #[serde(rename = "ID")]
    pub id: Option<u8>,
    pub cost: u64,
    pub count: u8,
}

#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum RestockVehicleEventType {
    Fighter(FighterType),
    SRV(SRVType),
}
