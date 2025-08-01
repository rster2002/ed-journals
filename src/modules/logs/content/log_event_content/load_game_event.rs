//! Fired when loading into the game.

use serde::{Deserialize, Serialize};

use crate::modules::ship::ShipType;

/// Fired when loading into the game.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadGameEvent {
    /// The name of the player.
    pub commander: String,

    /// The Frontier ID of the player.
    #[serde(rename = "FID")]
    pub fid: String,

    /// Whether horizons content has been enabled.
    pub horizons: bool,

    /// Whether odyssey content has been enabled.
    #[serde(default)]
    pub odyssey: bool,

    /// Information about the current active ship of the player.
    #[serde(flatten)]
    pub ship_info: Option<LoadGameEventShipInfo>,

    /// The gamemode the player loaded into.
    pub game_mode: Option<LoadGameEventGameMode>,

    /// The number of credits the player has in the bank.
    pub credits: u64,

    /// The number of credits the player has open in loans.
    pub loan: u64,
}

/// Information about the current active ship of the player.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct LoadGameEventShipInfo {
    /// The kind of ship currently active.
    pub ship: ShipType,

    /// The id of the active ship.
    #[serde(rename = "ShipID")]
    pub ship_id: u32,

    /// The name of the ship.
    pub ship_name: String,

    /// The id nameplate of the ship.
    pub ship_ident: String,

    /// The current fuel level of the active ship.
    pub fuel_level: f32,

    /// The max fuel capacity of the active ship.
    pub fuel_capacity: f32,
}

/// The gamemode the player loaded into.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LoadGameEventGameMode {
    Open,
    Solo,
    Group,
}

#[cfg(test)]
mod tests {
    use crate::logs::content::log_event_content::load_game_event::{
        LoadGameEvent, LoadGameEventGameMode, LoadGameEventShipInfo,
    };
    use crate::modules::ship::ShipType;

    #[test]
    fn load_game_event_is_parsed_correctly() {
        let parsed: LoadGameEvent = serde_json::from_str(
            r#"
            {
                "Commander": "HRC-2",
                "FID": "F44396",
                "Horizons": true,
                "Ship": "FerDeLance",
                "ShipID": 19,
                "ShipName": "jewel of parhoon",
                "ShipIdent": "hr-17f",
                "FuelLevel": 3.964024,
                "FuelCapacity": 8,
                "GameMode": "Open",
                "Credits": 2890718739,
                "Loan": 0
            }
        "#,
        )
        .unwrap();

        let expected = LoadGameEvent {
            commander: "HRC-2".to_string(),
            fid: "F44396".to_string(),
            horizons: true,
            odyssey: false,
            ship_info: Some(LoadGameEventShipInfo {
                ship: ShipType::FerDeLance,
                ship_id: 19,
                ship_name: "jewel of parhoon".to_string(),
                ship_ident: "hr-17f".to_string(),
                fuel_level: 3.964024,
                fuel_capacity: 8.0,
            }),
            game_mode: Some(LoadGameEventGameMode::Open),
            credits: 2890718739,
            loan: 0,
        };

        assert_eq!(parsed, expected);
    }
}
