use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachBodyEvent {
    star_system: String,
    body: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::logs::content::log_event_content::approach_body_event::ApproachBodyEvent;

    #[test]
    fn approach_body_is_parsed_correctly() {
        let value: ApproachBodyEvent = serde_json::from_value(json!({
            "StarSystem": "Eranin",
            "Body": "Eranin 2"
        }))
        .unwrap();

        let expected = ApproachBodyEvent {
            star_system: "Eranin".to_string(),
            body: "Eranin 2".to_string(),
        };

        assert_eq!(value, expected);
    }
}
