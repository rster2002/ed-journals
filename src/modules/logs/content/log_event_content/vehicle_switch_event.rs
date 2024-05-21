use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct VehicleSwitchEvent {
    pub to: VehicleSwitchEventTo,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum VehicleSwitchEventTo {
    Fighter,
    Mothership,
}
