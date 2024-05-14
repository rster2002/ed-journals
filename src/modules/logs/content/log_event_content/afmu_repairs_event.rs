use serde::{Serialize, Deserialize};
use crate::modules::models::ship::ship_module::ShipModule;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AFMURepairsEvent {
    pub module: ShipModule,

    #[serde(rename = "Module_Localised")]
    pub module_localized: Option<String>,
    pub fully_repaired: bool,
    pub health: f32,
}
