use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::modules::status::models::destination_status::DestinationStatus;
use crate::modules::status::models::flags::Flags;
use crate::modules::status::models::flags2::Flags2;
use crate::modules::status::models::fuel_status::FuelStatus;
use crate::modules::status::models::gui_focus::GuiFocus;
use crate::modules::status::models::legal_status::LegalStatus;
use crate::modules::status::models::planet_status::PlanetStatus;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "event")]
    pub event: String,
    pub flags: Flags,

    /// In some cases the status file might not contain any data.
    #[serde(flatten)]
    pub contents: Option<StatusContents>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StatusContents {
    pub flags2: Flags2,
    pub legal_state: LegalStatus,
    pub balance: u64,
}

// #[derive(Debug, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum StatusContents {
//     Ship(ShipStatus),
// }

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipStatus {
    pub pips: [u8; 3],
    pub fire_group: u8,
    pub gui_focus: GuiFocus,
    pub fuel: FuelStatus,
    pub cargo: f32,
    pub destination: Option<DestinationStatus>,

    #[serde(flatten)]
    pub planet_status: Option<PlanetStatus>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OnFootStatus {
    pub oxygen: f32,
    pub health: f32,
    pub temperature: f32,

    // TODO replace with enum
    pub selected_weapon: String,
    pub body_name: String,
}

impl ShipStatus {
    pub fn system_pips(&self) -> u8 {
        self.pips[0]
    }

    pub fn engine_pips(&self) -> u8 {
        self.pips[1]
    }

    pub fn weapon_pips(&self) -> u8 {
        self.pips[2]
    }
}
