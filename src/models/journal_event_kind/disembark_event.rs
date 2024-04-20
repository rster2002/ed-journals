use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct DisembarkEvent {
    #[serde(rename = "ID")]
    pub id: u32,
    pub star_system: String,
    pub system_address: u64,
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u32,
    pub on_station: bool,
    pub on_planet: bool,
    pub station_name: String,

    // TODO look into changing this into an enum
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "SRV")]
    pub srv: bool,
    pub taxi: bool,
    pub multicrew: bool,
}
