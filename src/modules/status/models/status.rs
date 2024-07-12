use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::modules::status::models::destination_status::DestinationStatus;
use crate::modules::status::models::flags2::Flags2;
use crate::modules::status::models::fuel_status::FuelStatus;
use crate::modules::status::models::gui_focus::GuiFocus;
use crate::modules::status::models::legal_status::LegalStatus;
use crate::modules::status::models::planet_status::PlanetStatus;
use crate::status::Flags;

/// Struct representing the live status of the player. Sometimes the file does exist, but might
/// not contain any data (for example when just starting the game.)
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Status {
    /// The timestamp when the status was updated.
    #[serde(rename = "timestamp")]
    pub timestamp: DateTime<Utc>,

    /// The event of the status. This is always set to 'Status'.
    #[serde(rename = "event")]
    pub event: String,

    /// In some cases the status file might not contain any data.
    #[serde(flatten)]
    pub contents: Option<StatusContents>,
}

/// The actual contents of the status file, containing flags for the different states part of the
/// ship can be in and might also contain information about the planet the player is currently close
/// to.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StatusContents {
    /// The current flags for the player. These flags are mostly for things that are related to
    /// the player's ship.
    pub flags: Flags,

    /// A second flags field which includes flags for the on-foot status of the player.
    pub flags2: Flags2,

    /// The current legal state of the player, indicating whether they are currently at risk of
    /// committing a crime.
    pub legal_state: LegalStatus,

    /// The current credit balance of the player.
    pub balance: u64,

    /// Information about the planet the player is currently close to.
    #[serde(flatten)]
    pub planet_status: Option<PlanetStatus>,

    /// Depending on the current state of the game, the status file includes information about
    /// either the ship of the player or information about the player when they're on-foot.
    #[serde(flatten)]
    pub kind: StatusKind,
}

/// The different sets of additional fields which are dependent on the current state of the player.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum StatusKind {
    /// Variant containing the fields for when the player is piloting a ship.
    Ship(ShipStatus),

    /// Variant containing the fields for when the player is on-foot.
    OnFoot(OnFootStatus),
}

impl StatusKind {
    /// Whether the current status kind is a ship status.
    pub fn is_ship_status(&self) -> bool {
        matches!(self, StatusKind::Ship(_))
    }

    /// Whether the current status kind is an on-foot status.
    pub fn is_on_foot_status(&self) -> bool {
        matches!(self, StatusKind::OnFoot(_))
    }
}

/// This model contains the fields which are included when the player is piloting a ship.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipStatus {
    /// The current status of the pips of the ship.
    pub pips: [u8; 3],

    /// The number of the fire-group which is currently selected.
    pub fire_group: u8,

    /// Which GUI currently has focus in the ship.
    pub gui_focus: GuiFocus,

    /// Information about the fuel of the current ship.
    pub fuel: FuelStatus,

    /// The number of tonnes of cargo the ship has on board.
    pub cargo: f32,

    /// Information about the currently targeted destination.
    pub destination: Option<DestinationStatus>,
}

impl ShipStatus {
    /// Returns the current number of pips that are set for the system category.
    pub fn system_pips(&self) -> u8 {
        self.pips[0]
    }

    /// Returns the current number of pips that are set for the engine category.
    pub fn engine_pips(&self) -> u8 {
        self.pips[1]
    }

    /// Returns the current number of pips that are set for the weapon category.
    pub fn weapon_pips(&self) -> u8 {
        self.pips[2]
    }
}

/// This model contains the fields which are included when the player is on-foot.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OnFootStatus {
    /// The percentage of oxygen the player currently has left.
    pub oxygen: f32,

    /// The percentage of health the player currently has left.
    pub health: f32,

    /// The current temperature of the player.
    pub temperature: f32,

    /// The name of the weapon the player currently has selected.
    // TODO replace with enum
    pub selected_weapon: String,
    pub selected_weapon_localized: Option<String>,
}
