use crate::materials::Material;
use crate::modules::galaxy::{
    AtmosphereType, PlanetClass, StarClass, StarLuminosity, VolcanismType,
};

#[derive(Debug)]
pub enum SpawnCondition {
    MinMeanTemperature(f32),
    MaxMeanTemperature(f32),
    NoAtmosphere,
    AnyThinAtmosphere,
    ThinAtmosphere(AtmosphereType),
    /// The minimum gravity the planet must have, in G
    MinGravity(f32),
    /// The maximum gravity the planet must have, in G
    MaxGravity(f32),
    PlanetClass(PlanetClass),
    MainStarClass(StarClass),
    ParentStarClass(StarClass),
    ParentStarLuminosity(StarLuminosity),
    MinOrEqualParentStarLuminosity(StarLuminosity),
    SystemContainsPlanetClass(PlanetClass),
    VolcanismType(VolcanismType),
    /// The minimum distance the planet needs to be from the sun in AU
    MinDistanceFromParentSun(f32),
    AnyVolcanism,

    MinPressure(f32),
    MaxPressure(f32),

    /// The maximum distance the planet needs to be from the center of a nebula in Ly
    WithinNebulaRange(f32),

    /// The target planet must have at least one geological signal present.
    GeologicalSignalsPresent,

    MaterialPresence(Material),

    RockyComposition,
    IcyComposition,
    MetalComposition,

    Special,

    Any(Vec<SpawnCondition>),
    All(Vec<SpawnCondition>),
}
