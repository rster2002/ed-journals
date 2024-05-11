use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct VehicleSwitchEvent {
    pub to: VehicleSwitchEventTo,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum VehicleSwitchEventTo {
    Fighter,
    Mothership,
}
