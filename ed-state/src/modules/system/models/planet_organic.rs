use ed_journals::exobiology::{Species, Variant};
use ed_journals::logs::scan_organic_event::ScanOrganicEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct PlanetOrganic {
    /// The specific species that has been scanned.
    pub species: Species,

    /// Specific species variant, if any.
    pub variant: Option<Variant>,

    /// The first active scan in the sequence.
    pub first_scan: Option<PlanetOrganicScan>,

    /// The second active scan performed for this sequence.
    pub second_scan: Option<PlanetOrganicScan>,

    /// The last active scan for this sequence.
    pub third_scan_scan: Option<PlanetOrganicScan>,

    /// The coordinates of the *player* when the organic was scanned.
    pub scan_locations: Vec<(f32, f32)>,
}

impl PlanetOrganic {
    pub fn is_completed(&self) -> bool {
        self.third_scan_scan.is_some()
    }

    pub fn progress_nr(&self) -> u8 {
        if self.third_scan_scan.is_some() {
            return 3;
        }

        if self.second_scan.is_some() {
            return 2;
        }

        1
    }

    pub fn progress_factor(&self) -> f64 {
        const ONE_THIRD: f64 = 1.0 / 3.0;

        if self.third_scan_scan.is_some() {
            return 1.0;
        }

        if self.second_scan.is_some() {
            return ONE_THIRD * 2.0;
        }

        ONE_THIRD
    }
}

impl From<Species> for PlanetOrganic {
    fn from(species: Species) -> Self {
        PlanetOrganic {
            species,
            variant: None,
            first_scan: None,
            second_scan: None,
            third_scan_scan: None,
            scan_locations: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlanetOrganicScan {
    pub scan: ScanOrganicEvent,
    pub location: Option<(f32, f32)>,
}
