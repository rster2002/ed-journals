use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CargoEvent {
    pub vessel: CargoEventVessel,
    pub inventory: Vec<CargoEventInventoryItem>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub enum CargoEventVessel {
    Ship,
    SRV,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CargoEventInventoryItem {
    pub name: String,
    pub count: u16,
    pub stolen: u16,
}
