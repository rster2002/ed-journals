use crate::shared::galaxy::atmosphere::Atmosphere;
use crate::shared::galaxy::atmosphere_type::AtmosphereType;
use crate::shared::galaxy::planet_class::PlanetClass;
use crate::shared::galaxy::star_class::StarClass;
use crate::shared::galaxy::star_luminosity::StarLuminosity;
use crate::shared::galaxy::volcanism_type::VolcanismType;

pub enum SpawnCondition {
    MinSurfaceTemperature(f32),
    MaxSurfaceTemperature(f32),
    NoAtmosphere,
    Atmosphere(Atmosphere),
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

    Any(Vec<SpawnCondition>),
    All(Vec<SpawnCondition>),
}
