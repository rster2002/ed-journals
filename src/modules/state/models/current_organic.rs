use serde::Serialize;

use crate::logs::scan_organic_event::ScanOrganicEvent;
use crate::modules::exobiology::Species;

#[derive(Debug, Serialize)]
pub struct CurrentOrganic {
    pub system_address: u64,
    pub body_id: u8,
    pub species: Species,
}

impl From<&ScanOrganicEvent> for CurrentOrganic {
    fn from(value: &ScanOrganicEvent) -> Self {
        CurrentOrganic {
            system_address: value.system_address,
            body_id: value.body,
            species: value.species.clone(),
        }
    }
}
