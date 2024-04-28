use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct VehicleSwitchEvent {
    pub to: VehicleSwitchEventTo,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum VehicleSwitchEventTo {
    Fighter,
    Mothership,
}
