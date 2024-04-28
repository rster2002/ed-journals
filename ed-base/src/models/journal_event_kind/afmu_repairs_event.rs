use crate::models::journal_event_kind::shared::ship::ship_module::ShipModule;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(rename_all = "PascalCase")]
pub struct AFMURepairsEvent {
    pub module: ShipModule,

    #[serde(rename = "Module_Localised")]
    pub module_localized: String,
    pub fully_repaired: bool,
    pub health: f32,
}
