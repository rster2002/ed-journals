use crate::modules::shared::ship::ship_module::ShipModule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct JetConeDamageEvent {
    pub module: ShipModule,
}
