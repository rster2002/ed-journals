use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LiftoffEvent {
    #[serde(flatten)]
    pub position: Option<LiftoffEventPosition>,
    pub star_system: Option<String>,
    pub system_address: Option<u64>,
    pub body: Option<String>,

    #[serde(rename = "BodyID")]
    pub body_id: Option<u64>,
    #[serde(default)]
    pub on_station: bool,
    #[serde(default)]
    pub on_planet: bool,
    #[serde(default)]
    pub multicrew: bool,
    pub nearest_destination: Option<String>,
    #[serde(default)]
    pub taxi: bool,
    pub player_controlled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LiftoffEventPosition {
    pub latitude: f32,
    pub longitude: f32,
}

#[cfg(test)]
mod tests {
    use crate::logs::content::log_event_content::liftoff_event::{
        LiftoffEvent, LiftoffEventPosition,
    };

    #[test]
    fn liftoff_event_is_parted_correctly() {
        let test_cases = [
            (
                r#"
                    {
                        "PlayerControlled": false,
                        "Taxi": false,
                        "Multicrew": false,
                        "StarSystem": "HIP 36731",
                        "SystemAddress": 251029096803,
                        "Body": "HIP 36731 3 c",
                        "BodyID": 26,
                        "OnStation": false,
                        "OnPlanet": true,
                        "Latitude": 14.493940,
                        "Longitude": 177.978470,
                        "NearestDestination": "Romano Industrial Forge"
                    }
                "#,
                LiftoffEvent {
                    position: Some(LiftoffEventPosition {
                        latitude: 14.493_94,
                        longitude: 177.978_47,
                    }),
                    star_system: Some("HIP 36731".to_string()),
                    system_address: Some(251029096803),
                    body: Some("HIP 36731 3 c".to_string()),
                    body_id: Some(26),
                    on_station: false,
                    on_planet: true,
                    multicrew: false,
                    nearest_destination: Some("Romano Industrial Forge".to_string()),
                    taxi: false,
                    player_controlled: false,
                },
            ),
            (
                r#"
                    {
                        "timestamp": "2024-01-08T18:47:23Z",
                        "event": "Liftoff",
                        "PlayerControlled": true,
                        "Taxi": false,
                        "Multicrew": false,
                        "StarSystem": "Gludgae XP-E d12-0",
                        "SystemAddress": 11232479211,
                        "Body": "Gludgae XP-E d12-0 8 a",
                        "BodyID": 11,
                        "OnStation": false,
                        "OnPlanet": true,
                        "Latitude": -63.379852,
                        "Longitude": 5.273848
                    }
                "#,
                LiftoffEvent {
                    position: Some(LiftoffEventPosition {
                        latitude: -63.379852,
                        longitude: 5.273848,
                    }),
                    star_system: Some("Gludgae XP-E d12-0".to_string()),
                    system_address: Some(11232479211),
                    body: Some("Gludgae XP-E d12-0 8 a".to_string()),
                    body_id: Some(11),
                    on_station: false,
                    on_planet: true,
                    multicrew: false,
                    nearest_destination: None,
                    taxi: false,
                    player_controlled: true,
                },
            ),
        ];

        for (test, expected) in test_cases {
            let value: LiftoffEvent = serde_json::from_str(test).unwrap();

            assert_eq!(value, expected);
        }
    }
}
