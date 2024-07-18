use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TouchdownLocation {
    pub system_address: u64,
    pub body_id: u8,
    pub coordinates: (f32, f32),
}
