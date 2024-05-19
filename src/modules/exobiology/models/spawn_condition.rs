use crate::modules::galaxy::{AtmosphereType, PlanetClass, StarClass, StarLuminosity, VolcanismType};

pub enum SpawnCondition {
    MinMeanTemperature(f32),
    MaxMeanTemperature(f32),
    NoAtmosphere,
    AnyThinAtmosphere,
    ThinAtmosphere(AtmosphereType),
    MinGravity(f32),
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

    WithinNebulaRange(f32),

    /// The target planet must have at least one geological signal present.
    GeologicalSignalsPresent,

    Any(Vec<SpawnCondition>),
    All(Vec<SpawnCondition>),
}
