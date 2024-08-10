use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
#[derive(Default)]
pub struct MissionState {
    pub missions: HashMap<u64, Mission>,
}

#[derive(Serialize)]
pub struct Mission {}

