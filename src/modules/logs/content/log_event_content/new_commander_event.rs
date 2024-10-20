use serde::{Deserialize, Serialize};

/// Fired when the player starts as a new commander.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NewCommanderEvent {
    /// The name of the newly created commander.
    pub name: String,

    /// The Frontier ID.
    #[serde(rename = "FID")]
    pub fid: String,

    /// The name of the package the new commander will deploy with.
    pub package: String,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::modules::logs::content::log_event_content::new_commander_event::NewCommanderEvent;

    #[test]
    fn new_commander_event_is_parsed_correctly() {
        let value: NewCommanderEvent = serde_json::from_value(json!({
            "Name": "HRC1",
            "FID": "F44396",
            "Package": "ImperialBountyHunter"
        }))
        .unwrap();

        let expected = NewCommanderEvent {
            name: "HRC1".to_string(),
            fid: "F44396".to_string(),
            package: "ImperialBountyHunter".to_string(),
        };

        assert_eq!(value, expected);
    }
}
