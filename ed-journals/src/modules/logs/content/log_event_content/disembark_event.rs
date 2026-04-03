//! Fired when the player disembarks from their ship.

use serde::{Deserialize, Serialize};

use crate::modules::station::StationType;

/// Fired when the player disembarks from their ship.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DisembarkEvent {
    // TODO check when this is optional
    /// Currently unknown that this value means. If you know please let me know by
    /// [opening an issue](https://github.com/rster2002/ed-journals/issues/new).
    #[serde(rename = "ID")]
    pub id: Option<u32>,

    /// The name of the star system the player is currently in.
    pub star_system: String,

    /// The address of the current system.
    pub system_address: u64,

    /// The name of the body the player is currently on.
    pub body: String,

    /// The id of the body player is currently on.
    #[serde(rename = "BodyID")]
    pub body_id: u32,

    /// Whether the player is currently on a station.
    pub on_station: bool,

    /// Whether the player is currently on a planet.
    pub on_planet: bool,

    // TODO probably only set when [on_station] is true.
    /// The name of the station the player is currently on.
    pub station_name: Option<String>,

    // TODO probably only set when [on_station] is true.
    /// The type of the station the player is currently on.
    pub station_type: Option<StationType>,

    // TODO probably only set when [on_station] is true.
    /// The market id of the station the player is currently on.
    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

    /// Whether the player has embarked to an SRV.
    #[serde(rename = "SRV")]
    pub srv: bool,

    /// Whether the player has embarked a taxi.
    #[serde(default)]
    pub taxi: bool,

    /// Whether the player has embarked multicrew.
    #[serde(default)]
    pub multicrew: bool,
}
