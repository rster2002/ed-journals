use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct MissionState {
    pub missions: HashMap<u64, Mission>,
}

#[derive(Serialize)]
pub struct Mission {}

impl Default for MissionState {
    fn default() -> Self {
        MissionState {
            missions: HashMap::new(),
        }
    }
}
