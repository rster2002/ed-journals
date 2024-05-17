use serde::{Serialize, Deserialize};
use crate::models::ship::ship_type::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadGameEvent {
    pub commander: String,

    #[serde(rename = "FID")]
    pub fid: String,
    pub horizons: bool,

    #[serde(default)]
    pub odyssey: bool,

    #[serde(flatten)]
    pub ship_info: Option<LoadGameEventShipInfo>,
    pub game_mode: Option<LoadGameEventGameMode>,
    pub credits: u64,
    pub loan: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LoadGameEventShipInfo {
    pub ship: ShipType,

    #[serde(rename = "ShipID")]
    pub ship_id: u32,
    pub ship_name: String,
    pub ship_ident: String,
    pub fuel_level: f32,
    pub fuel_capacity: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum LoadGameEventGameMode {
    Open,
    Solo,
    Group,
}

#[cfg(test)]
mod tests {
    use crate::logs::content::log_event_content::load_game_event::{LoadGameEvent, LoadGameEventGameMode, LoadGameEventShipInfo};
    use crate::models::ship::ship_type::ShipType;

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
            odyssey: true,
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
