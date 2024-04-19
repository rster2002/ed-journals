use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadGameEvent {
    pub commander: String,

    #[serde(rename = "FID")]
    pub fid: String,
    pub horizons: bool,
    pub commander: String,
    pub commander: String,
    pub commander: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::models::journal_event_kind::load_game_event::LoadGameEvent;

    fn load_game_event_is_parsed_correctly() {
        let parsed: LoadGameEvent = serde_json::from_value(json!({
            "timestamp": "2017-02-10T14:25:51Z",
            "event": "LoadGame",
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
        }))
            .unwrap();

        let expected = LoadGameEvent;

        assert_eq!(parsed, expected);
    }
}
