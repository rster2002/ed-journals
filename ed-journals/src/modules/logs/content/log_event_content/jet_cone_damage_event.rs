use crate::modules::shared::ship::ship_module::ShipModule;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct JetConeDamageEvent {
    pub module: ShipModule,
}