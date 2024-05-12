use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::modules::status::models::destination_status::DestinationStatus;
use crate::modules::status::models::flags2::Flags2;
use crate::modules::status::models::flags::Flags;
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
    pub flags2: Flags2,
    pub pips: [u8; 3],
    pub fire_group: u8,
    pub gui_focus: GuiFocus,
    pub fuel: FuelStatus,
    pub cargo: f32,
    pub legal_state: LegalStatus,
    pub balance: u64,
    pub destination: Option<DestinationStatus>,

    #[serde(flatten)]
    pub planet_status: Option<PlanetStatus>,
}

impl Status {
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

