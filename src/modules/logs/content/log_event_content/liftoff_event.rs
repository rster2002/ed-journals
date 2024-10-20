use serde::{Deserialize, Serialize};

/// Fired when lifting off from a planet.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LiftoffEvent {
    /// The coordinates where the player has lifted off when on a planet.
    #[serde(flatten)]
    pub position: Option<LiftoffEventPosition>,

    /// The name of the star system the player is currently in.
    pub star_system: String,

    /// The address of the star system the player is currently in.
    pub system_address: u64,

    /// The name of the body the player lifting off from.
    pub body: String,

    /// The id of the body the player lifting off from.
    #[serde(rename = "BodyID")]
    pub body_id: u64,

    /// Whether the player is lifting off from a settlement.
    pub on_station: bool,

    /// Whether the player is lifting off from the surface of the planet and not from a landing pad.
    pub on_planet: bool,

    /// Whether the player is currently in multicrew.
    pub multicrew: bool,

    /// Name of the nearest destination.
    pub nearest_destination: Option<String>,

    /// Whether the liftoff is fired from a taxi ship.
    pub taxi: bool,

    /// Whether the player is controlling the ship.
    pub player_controlled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct LiftoffEventPosition {
    pub latitude: f32,
    pub longitude: f32,
}

impl Into<(f32, f32)> for LiftoffEventPosition {
    fn into(self) -> (f32, f32) {
        (self.latitude, self.longitude)
    }
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
            let value: LiftoffEvent = serde_json::from_str(test).unwrap();

            assert_eq!(value, expected);
        }
    }
}
