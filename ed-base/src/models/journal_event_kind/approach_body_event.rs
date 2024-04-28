use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ApproachBodyEvent {
    star_system: String,
    body: String,
}

#[cfg(test)]
mod tests {
    use crate::models::journal_event_kind::approach_body_event::ApproachBodyEvent;
    use serde_json::json;

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
