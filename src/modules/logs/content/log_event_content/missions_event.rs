use serde::{Deserialize, Serialize};

use crate::modules::station::MissionType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionsEvent {
    pub active: Vec<MissionEventEntry>,
    pub failed: Vec<MissionEventEntry>,
    pub complete: Vec<MissionEventEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MissionEventEntry {
    #[serde(rename = "MissionID")]
    pub mission_id: u64,
    pub name: MissionType,
    pub passenger_mission: bool,
    pub expires: u64,
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::logs::content::log_event_content::missions_event::{
        MissionEventEntry, MissionsEvent,
    };
    use crate::modules::station::MissionType;

    #[test]
    fn mission_event_is_parsed_correctly() {
        let value: MissionsEvent = serde_json::from_value(json!({
            "Active": [
                {
                    "MissionID": 65380900,
                    "Name": "Mission_Courier_name",
                    "PassengerMission": false,
                    "Expires": 82751
                }
            ],
            "Failed": [],
            "Complete": []
        }))
        .unwrap();

        let expected = MissionsEvent {
            active: vec![MissionEventEntry {
                mission_id: 65380900,
                name: MissionType("Mission_Courier_name".to_string()),
                passenger_mission: false,
                expires: 82751,
            }],
            failed: vec![],
            complete: vec![],
        };

        assert_eq!(value, expected);
    }
}
