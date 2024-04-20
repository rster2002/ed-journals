use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct CodexEntryEvent {
    #[serde(rename = "EntryID")]
    pub entry_id: u64,

    // TODO turn this into an enum
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localized: String,

    // TODO turn this into an enum
    pub sub_category: String,

    #[serde(rename = "SubCategory_Localised")]
    pub sub_category_localized: String,

    // TODO turn this into an enum
    pub category: String,

    #[serde(rename = "Category_Localised")]
    pub category_localized: String,

    // TODO turn this into an enum
    pub region: String,

    #[serde(rename = "Region_Localised")]
    pub region_localized: String,

    pub system: String,
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u8,

    // TODO sometimes missing, sometimes an empty string
    pub nearest_destination: Option<String>,

    // TODO check when this is filled
    pub latitude: Option<f32>,

    // TODO check when this is filled
    pub longitude: Option<f32>,
    pub is_new_entry: bool,
}
