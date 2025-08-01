use serde::{Deserialize, Serialize};

use crate::ship::ShipType;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "PascalCase")]
pub struct ShipyardRedeem {
    pub ship_type: ShipType,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localized: Option<String>,

    #[serde(rename = "BundleID")]
    pub bundle_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}
