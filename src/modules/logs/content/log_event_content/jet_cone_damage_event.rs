use serde::{Serialize, Deserialize};
use crate::modules::ship::ShipModule;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct JetConeDamageEvent {
    pub module: ShipModule,
}
