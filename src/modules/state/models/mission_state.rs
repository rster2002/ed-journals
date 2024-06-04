use std::collections::HashMap;
use serde::Serialize;

#[derive(Serialize)]
pub struct MissionState {
    pub missions: HashMap<u64, Mission>,
}

#[derive(Serialize)]
pub struct Mission {

}

impl Default for MissionState {
    fn default() -> Self {
        MissionState {
            missions: HashMap::new(),
        }
    }
}
