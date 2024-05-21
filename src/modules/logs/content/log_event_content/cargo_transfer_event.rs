use serde::{Deserialize, Serialize};

use crate::modules::trading::Commodity;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEvent {
    pub transfers: Vec<CargoTransferEventTransfer>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEventTransfer {
    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
    pub direction: CargoTransferEventTransferDirection,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CargoTransferEventTransferDirection {
    #[serde(rename = "tocarrier")]
    ToCarrier,

    #[serde(rename = "toship")]
    ToShip,

    #[serde(rename = "tosrv")]
    ToSRV,
}
