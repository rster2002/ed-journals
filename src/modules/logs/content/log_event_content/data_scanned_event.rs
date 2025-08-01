//! Fired when scanning data while in a ship or SRV.

use serde::{Deserialize, Serialize};

/// Fired when scanning data while in a ship or SRV.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct DataScannedEvent {
    /// The type of datapoint scanned.
    #[serde(rename = "Type")]
    pub kind: DataScannedEventType,
}

/// The type of datapoint scanned.
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum DataScannedEventType {
    #[serde(rename = "$Datascan_AbandonedDataLog;")]
    AbandonedDataLog,

    #[serde(rename = "$Datascan_DataLink;")]
    DataLink,

    #[serde(rename = "$Datascan_DataPoint;")]
    DataPoint,

    #[serde(rename = "$Datascan_ListeningPost;")]
    ListeningPost,

    #[serde(rename = "$Datascan_ShipUplink;")]
    ShipUplink,

    #[serde(rename = "$Datascan_WreckedShip;")]
    WreckedShip,

    #[serde(rename = "$Datascan_TouristBeacon;")]
    TouristBeacon,

    #[serde(rename = "$Datascan_ANCIENTCODEX;")]
    AncientCodex,

    #[serde(rename = "$Datascan_Unknown_Uplink;")]
    UnknownUplink,

    #[serde(rename = "$Datascan_Settlement_Unknown;")]
    SettlementUnknown,
}
