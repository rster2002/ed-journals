use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoEvent {
    pub vessel: CargoEventVessel,

    // TODO this seems to be missing for SRV?
    #[serde(default)]
    pub inventory: Vec<CargoEventInventoryItem>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub enum CargoEventVessel {
    Ship,
    SRV,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoEventInventoryItem {
    pub name: String,
    pub count: u16,
    pub stolen: u16,
}
