use ed_journals::galaxy::{Atmosphere, Gravity, PlanetClass, PlanetComposition, Volcanism};
use ed_journals::logs::scan_event::ScanEventParent;
use ed_journals::materials::Material;
use std::collections::HashSet;

/// Information about a planet.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetPlanet {
    /// The class of the planet.
    pub class: PlanetClass,

    /// The atmosphere of the planet.
    pub atmosphere: Atmosphere,

    /// The surface gravity of the planet.
    pub surface_gravity: Gravity,

    /// The surface temperature of the planet.
    pub surface_temperature: f32,

    /// The volcanism on the planet.
    pub volcanism: Volcanism,

    /// Set of unique materials present on the planet.
    pub materials: HashSet<Material>,

    /// Composition values of the planet.
    pub composition: Option<PlanetComposition>,

    /// The parents the planet orbits around.
    pub parents: Vec<ScanEventParent>,

    /// The semi-major axis of the planet.
    pub semi_major_axis: f32,

    /// Whether there are any geological signals present on the planet.
    pub geological_signals_present: bool,
}
