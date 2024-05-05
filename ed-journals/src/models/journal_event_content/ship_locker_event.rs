use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLockerEvent {
    #[serde(flatten)]
    pub contents: Option<ShipLockerEventContents>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLockerEventContents {
    pub items: Vec<ShipLockerEventObject>,
    pub components: Vec<ShipLockerEventObject>,
    pub consumables: Vec<ShipLockerEventObject>,
    pub data: Vec<ShipLockerEventObject>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ShipLockerEventObject {
    pub name: String,
    pub owner_id: u64,

    #[serde(rename = "MissionID")]
    pub mission_id: Option<u64>,
    pub count: u16,
}
