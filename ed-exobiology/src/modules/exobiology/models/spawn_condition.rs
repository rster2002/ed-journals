use ed_journals::galaxy::{AtmosphereType, PlanetClass, StarClass, StarLuminosity, VolcanismType};
use ed_journals::materials::Material;

#[derive(Debug)]
pub enum SpawnCondition<'a> {
    MinMeanTemperature(f32),
    MaxMeanTemperature(f32),
    NoAtmosphere,
    AnyThinAtmosphere,
    ThinAtmosphere(AtmosphereType),

    /// The minimum gravity the planet must have, in Gs
    MinGravity(f32),

    /// The maximum gravity the planet must have, in Gs
    MaxGravity(f32),
    PlanetClass(PlanetClass),
    ParentStarClass(StarClass),
    ParentStarLuminosity(StarLuminosity),
    MinOrEqualParentStarLuminosity(StarLuminosity),
    SystemContainsPlanetClass(PlanetClass),
    VolcanismType(VolcanismType),
    /// The minimum distance the planet needs to be from the sun in AU
    MinDistanceFromParentSun(f32),
    AnyVolcanism,

    /// The maximum distance the planet needs to be from the center of a nebula in Ly
    WithinNebulaRange(f32),

    /// The target planet must have at least one geological signal present.
    GeologicalSignalsPresent,

    MaterialPresence(Material),

    RockyComposition,
    IcyComposition,
    MetalComposition,

    Special,

    Any(&'a [SpawnCondition<'a>]),
    All(&'a [SpawnCondition<'a>]),
}
