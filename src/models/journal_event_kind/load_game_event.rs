use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LoadGameEvent {
    pub commander: String,

    #[serde(rename = "FID")]
    pub fid: String,
    pub horizons: bool,

    #[serde(default)]
    pub odyssey: bool,
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u32,
    pub ship_name: String,
    pub ship_ident: String,
    pub fuel_level: f32,
    pub fuel_capacity: f32,
    pub game_mode: String,
    pub credits: u64,
    pub loan: u64,
}

#[cfg(test)]
mod tests {

    use crate::models::journal_event_kind::load_game_event::LoadGameEvent;

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
            ship: "FerDeLance".to_string(),
            ship_id: 19,
            ship_name: "jewel of parhoon".to_string(),
            ship_ident: "hr-17f".to_string(),
            fuel_level: 3.964024,
            fuel_capacity: 8.0,
            game_mode: "Open".to_string(),
            credits: 2890718739,
            loan: 0,
        };

        assert_eq!(parsed, expected);
    }
}
