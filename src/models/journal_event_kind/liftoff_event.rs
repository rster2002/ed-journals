use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LiftoffEvent {
    #[serde(flatten)]
    pub position: Option<LiftoffEventPosition>,
    pub star_system: String,
    pub system_address: u64,
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,
    pub on_station: bool,
    pub on_planet: bool,
    pub multicrew: bool,
    pub nearest_destination: Option<String>,
    pub taxi: bool,
    pub player_controlled: bool,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct LiftoffEventPosition {
    pub latitude: f32,
    pub longitude: f32,
}

#[cfg(test)]
mod tests {
    use crate::models::journal_event_kind::liftoff_event::{LiftoffEvent, LiftoffEventPosition};

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
                        latitude: 14.493940,
                        longitude: 177.978470,
                    }),
                    star_system: "HIP 36731".to_string(),
                    system_address: 251029096803,
                    body: "HIP 36731 3 c".to_string(),
                    body_id: 26,
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
                    star_system: "Gludgae XP-E d12-0".to_string(),
                    system_address: 11232479211,
                    body: "Gludgae XP-E d12-0 8 a".to_string(),
                    body_id: 11,
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
            let value: LiftoffEvent = serde_json::from_str(test)
                .unwrap();

            assert_eq!(value, expected);
        }
    }
}
