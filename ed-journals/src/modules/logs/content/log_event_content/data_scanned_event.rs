use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DataScannedEvent {
    #[serde(rename = "Type")]
    pub kind: DataScannedEventType,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
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
}
