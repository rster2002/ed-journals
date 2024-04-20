use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BuySuitEvent {
    // TODO check if this can be an enum
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,
    pub price: u64,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,
    pub suit_mods: Vec<BuySuitEventSuitMod>,
}

// TODO not sure what this should contain
// TODO this might be the same as in suit_loadout_event
#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct BuySuitEventSuitMod {}
