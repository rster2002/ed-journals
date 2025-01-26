use crate::materials::Material;
use crate::modules::galaxy::{
    AtmosphereType, PlanetClass, StarClass, StarLuminosity, VolcanismType,
};

#[derive(Debug)]
pub enum SpawnCondition<'a> {
    /// The minimum temperature a planet must have to be matched.
    MinMeanTemperature(f32),

    /// The maximum temperature a planet can have to be matched.
    MaxMeanTemperature(f32),

    /// Specifies that the planet MUST NOT have any atmosphere.
    NoAtmosphere,

    /// Specifies that the planet must have a thin atmosphere, but not which kind of atmosphere it
    /// must be.
    AnyThinAtmosphere,

    /// Checks that the planet has a thin atmosphere and ensures that the atmosphere is of the given
    /// type.
    ThinAtmosphere(AtmosphereType),

    /// The minimum gravity the planet must have, in G
    MinGravity(f32),

    /// The maximum gravity the planet must have, in G
    MaxGravity(f32),

    /// Specifies that the planet must be of the give planet class.
    PlanetClass(PlanetClass),

    /// Specifies that the planet must be of the give planet class.
    ParentStarClass(StarClass),

    /// Checks that the parent star of the planet has a given luminosity.
    ParentStarLuminosity(StarLuminosity),

    /// Checks that the parent star of the planet has at least the given luminosity.
    MinOrEqualParentStarLuminosity(StarLuminosity),

    /// Checks that the system has at least one planet of the given planet class, which could also
    /// be the planet that is being checked.
    SystemContainsPlanetClass(PlanetClass),

    /// Specifies that any kind of volcanism needs to be present on the planet.
    AnyVolcanism,

    /// Checks that the given volcanism type is present on the planet.
    VolcanismType(VolcanismType),

    /// The minimum distance the planet needs to be from the sun in AU
    MinDistanceFromParentSun(f32),

    /// The maximum distance the planet needs to be from the center of a nebula in Ly
    WithinNebulaRange(f32),

    /// The target planet must have at least one geological signal present.
    GeologicalSignalsPresent,

    MaterialPresence(Material),

    RockyComposition,
    IcyComposition,
    MetalComposition,

    /// The species has a special spawn condition that cannot be checked. This will cause the whole
    /// check to fail and not show up in the predictions.
    Special,

    /// Check that at least one condition passes.
    Any(&'a [SpawnCondition<'a>]),

    /// Checks that all conditions pass.
    All(&'a [SpawnCondition<'a>]),
}
