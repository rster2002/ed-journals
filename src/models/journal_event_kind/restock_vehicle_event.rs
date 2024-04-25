use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct RestockVehicleEvent {
    #[serde(rename = "Type")]
    pub kind: RestockVehicleEventType,

    #[serde(rename = "Type_Localised")]
    pub type_localized: String,
    pub loadout: RestockVehicleEventLoadout,

    #[serde(rename = "ID")]
    pub id: u8,
    pub cost: u64,
    pub count: u8,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RestockVehicleEventType {
    #[serde(rename = "testbuggy")]
    SRV,

    #[serde(rename = "independent_fighter")]
    TaipanFighter,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum RestockVehicleEventLoadout {
    #[serde(rename = "zero")]
    Zero,

    #[serde(rename = "starter")]
    Starter,
}
