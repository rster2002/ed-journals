use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredModulesEvent {
    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub station_name: String,
    pub star_system: String,
    pub items: Vec<StoredModulesEventItem>,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct StoredModulesEventItem {
    // TODO look into replacing this with an enum
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub storage_slot: u16,
    pub star_system: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
    pub transfer_cost: u64,
    pub transfer_time: u32,
    pub buy_price: u64,
    pub hot: bool,
}
