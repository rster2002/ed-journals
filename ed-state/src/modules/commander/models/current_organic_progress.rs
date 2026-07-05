use ed_journals::exobiology::Genus;
use ed_journals::logs::scan_organic_event::ScanOrganicEvent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentOrganicProgress {
    pub system_address: u64,
    pub body_id: u8,
    pub genus: Genus,
}

impl From<&ScanOrganicEvent> for CurrentOrganicProgress {
    fn from(value: &ScanOrganicEvent) -> Self {
        CurrentOrganicProgress {
            system_address: value.system_address,
            body_id: value.body,
            genus: value.genus.clone(),
        }
    }
}

impl From<ScanOrganicEvent> for CurrentOrganicProgress {
    fn from(value: ScanOrganicEvent) -> Self {
        Self::from(&value)
    }
}
