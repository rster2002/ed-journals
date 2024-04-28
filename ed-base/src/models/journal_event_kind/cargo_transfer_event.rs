use crate::models::journal_event_kind::shared::trading::commodity::Commodity;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEvent {
    pub transfers: Vec<CargoTransferEventTransfer>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CargoTransferEventTransfer {
    #[serde(rename = "Type")]
    pub kind: Commodity,
    pub count: u16,
    pub direction: CargoTransferEventTransferDirection,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub enum CargoTransferEventTransferDirection {
    #[serde(rename = "tocarrier")]
    ToCarrier,

    #[serde(rename = "toship")]
    ToShip,

    #[serde(rename = "tosrv")]
    ToSRV,
}
