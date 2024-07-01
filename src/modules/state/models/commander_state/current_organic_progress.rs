use serde::Serialize;

use crate::logs::scan_organic_event::ScanOrganicEvent;
use crate::modules::exobiology::Species;

#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct CurrentOrganicProgress {
    pub system_address: u64,
    pub body_id: u8,
    pub species: Species,
    pub first_scan: ScanOrganicEvent,
    pub second_scan: Option<ScanOrganicEvent>,
}

impl From<&ScanOrganicEvent> for CurrentOrganicProgress {
    fn from(value: &ScanOrganicEvent) -> Self {
        CurrentOrganicProgress {
            system_address: value.system_address,
            body_id: value.body,
            species: value.species.clone(),
            first_scan: value.clone(),
            second_scan: None,
        }
    }
}
