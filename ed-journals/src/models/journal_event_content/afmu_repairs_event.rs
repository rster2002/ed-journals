use crate::models::journal_event_content::shared::ship::ship_module::ShipModule;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AFMURepairsEvent {
    pub module: ShipModule,

    #[serde(rename = "Module_Localised")]
    pub module_localized: Option<String>,
    pub fully_repaired: bool,
    pub health: f32,
}
