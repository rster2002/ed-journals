use crate::models::journal_event_content::shared::trading::commodity::Commodity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEvent {
    pub transfers: Vec<CargoTransferEventTransfer>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEventTransfer {
    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
    pub direction: CargoTransferEventTransferDirection,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum CargoTransferEventTransferDirection {
    #[serde(rename = "tocarrier")]
    ToCarrier,

    #[serde(rename = "toship")]
    ToShip,

    #[serde(rename = "tosrv")]
    ToSRV,
}
