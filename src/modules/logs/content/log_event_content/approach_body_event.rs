//! Fired whenever the player approaches a body.

use serde::{Deserialize, Serialize};

/// Fired whenever the player approaches a body. This is usually when the game also performs a scan
/// which fires a [ScanEvent](crate::logs::scan_event::ScanEvent).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ApproachBodyEvent {
    /// The star system the approached body is part of.
    pub star_system: String,

    /// The name of the body which the player is approaching.
    pub body: String,
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
