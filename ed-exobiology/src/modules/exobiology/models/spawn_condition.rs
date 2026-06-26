use ed_journals::galaxy::{AtmosphereType, PlanetClass, Region, StarClass, StarLuminosity, VolcanismType};
use ed_journals::materials::Material;

/// A condition that the planet must meet to spawn a specific species.
#[derive(Debug)]
pub enum SpawnCondition<'a> {
    /// Matches any planet that has at least the given mean temperature in Celsius.
    MinMeanTemperature(f32),

    /// Matches any planet that has at most the given mean temperature in Celsius.
    MaxMeanTemperature(f32),

    /// Matches any planet that has no atmosphere.
    NoAtmosphere,

    /// Matches any planet that has at least one thin atmosphere.
    AnyThinAtmosphere,

    /// Matches any planet that has a thin atmosphere of the given type.
    ThinAtmosphere(AtmosphereType),

    /// Matches any planet that has at least the given gravity, in Gs.
    MinGravity(f32),

    /// Matches any planet that has at most the given gravity, in Gs.
    MaxGravity(f32),

    /// Matches any planet that has the given class.
    PlanetClass(PlanetClass),

    /// Matches any planet that orbits a star of the given class.
    ParentStarClass(StarClass),

    /// The class of the primary star in the system.
    PrimaryStarClass(StarClass),

    /// Matches any planet that orbits a star of the given luminosity.
    ParentStarLuminosity(StarLuminosity),

    /// Matches any planet that orbits a star of the given luminosity or higher.
    MinOrEqualParentStarLuminosity(StarLuminosity),

    /// Matches any planet in a system that contains at least one planet of the given class.
    SystemContainsPlanetClass(PlanetClass),

    /// Matches any planet that has any type of volcanism.
    AnyVolcanism,

    /// Matches any planet that has a volcanism of the given type.
    VolcanismType(VolcanismType),

    /// The minimum atmospheric pressure required.
    MinPressure(f32),

    /// The maximum atmospheric pressure.
    MaxPressure(f32),

    /// Matches any planet that is at least the given distance from the center of the sun in AU.
    MinDistanceFromParentSun(f32),

    /// The maximum distance the planet needs to be from the center of a nebula in Ly
    WithinNebulaRange(f32),

    /// Matches any planet that has at least one geological signal present.
    GeologicalSignalsPresent,

    /// Matches any planet that has at least one material of the given type.
    MaterialPresence(Material),

    RockyComposition,
    IcyComposition,
    MetalComposition,

    /// The region of the galaxy the species can spawn in.
    Region(Region),

    /// Special conditions that cannot be checked in the journal state. Species with this condition
    /// will never be matched automatically.
    Special,

    /// A planet must satisfy at least one of the given conditions.
    Any(&'a [SpawnCondition<'a>]),

    /// A planet must satisfy all the given conditions.
    All(&'a [SpawnCondition<'a>]),
}
