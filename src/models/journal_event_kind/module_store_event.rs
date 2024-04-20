use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct ModuleStoreEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,

    // TODO maybe replace this with an enum or a struct
    pub slot: String,

    // TODO maybe replace this with an enum or a struct
    pub stored_item: String,

    #[serde(rename = "StoredItem_Localised")]
    pub stored_item_localized: String,

    // TODO maybe replace this with an enum
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u8,
    pub hot: bool,
}
