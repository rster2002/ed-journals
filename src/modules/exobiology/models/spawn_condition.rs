use crate::models::materials::material::Material;
use crate::modules::models::galaxy::atmosphere_type::AtmosphereType;
use crate::modules::models::galaxy::planet_class::PlanetClass;
use crate::modules::models::galaxy::star_class::StarClass;
use crate::modules::models::galaxy::star_luminosity::StarLuminosity;
use crate::modules::models::galaxy::volcanism_type::VolcanismType;

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

    MaterialPresence(Material),

    RockyComposition,

    Any(Vec<SpawnCondition>),
    All(Vec<SpawnCondition>),
}
