use ed_journals::galaxy::{StarClass, StarLuminosity};
use ed_journals::logs::scan_event::ScanEventStar;

/// Information about a star in a system.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SpawnSourceStar {
    /// The class of a given star.
    pub class: StarClass,

    /// The luminosity of a given star.
    pub luminosity: StarLuminosity,
}

impl From<ScanEventStar> for SpawnSourceStar {
    fn from(value: ScanEventStar) -> Self {
        SpawnSourceStar {
            class: value.star_type,
            luminosity: value.luminosity,
        }
    }
}
