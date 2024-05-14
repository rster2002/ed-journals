use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PowerplayEvent {
    pub power: String,
    pub rank: u8,
    pub merits: u32,
    pub votes: u32,
    pub time_pledged: u32,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::modules::logs::content::log_event_content::powerplay_event::PowerplayEvent;

    #[test]
    fn powerplay_event_is_parsed_correctly() {
        let value: PowerplayEvent = serde_json::from_value(json!({
            "Power": "Edmund Mahon",
            "Rank": 0,
            "Merits": 10,
            "Votes": 0,
            "TimePledged": 433024
        }))
        .unwrap();

        let expected = PowerplayEvent {
            power: "Edmund Mahon".to_string(),
            rank: 0,
            merits: 10,
            votes: 0,
            time_pledged: 433024,
        };

        assert_eq!(value, expected)
    }
}
