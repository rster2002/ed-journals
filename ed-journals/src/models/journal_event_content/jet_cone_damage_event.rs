use serde::Deserialize;
use crate::journal_event_content::shared::ship::ship_module::ShipModule;

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct JetConeDamageEvent {
    pub module: ShipModule,
}
