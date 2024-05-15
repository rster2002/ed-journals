use crate::logs::content::log_event_content::scan_event::ScanEventPlanet;
use crate::models::galaxy::atmosphere::AtmosphereDensity;
use crate::modules::models::galaxy::atmosphere_type::AtmosphereType;
use crate::modules::models::galaxy::planet_class::PlanetClass;
use crate::modules::models::galaxy::star_class::StarClass;
use crate::modules::models::galaxy::star_luminosity::StarLuminosity;
use crate::modules::models::galaxy::volcanism_type::VolcanismType;

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

impl SpawnCondition {
    // FIXME: Probably need to use more information that just a ScanEventPlanet,
    //  as we also need information about the parent star,
    //  and the GeologicalSignalsPresent condition is only available after mapping the planet.
    /// Checks if the given planet scan meets the spawn condition.
    pub fn check_for_planet(&self, planet_scan: &ScanEventPlanet) -> bool {
        match self {
            SpawnCondition::MinMeanTemperature(min_temp) => {
                planet_scan.surface_temperature >= *min_temp
            }
            SpawnCondition::MaxMeanTemperature(max_temp) => {
                planet_scan.surface_temperature <= *max_temp
            }
            SpawnCondition::NoAtmosphere => planet_scan.atmosphere_type == None,
            SpawnCondition::AnyThinAtmosphere => {
                planet_scan.atmosphere.density == AtmosphereDensity::Thin
            }
            SpawnCondition::ThinAtmosphere(atmosphere_type) => {
                planet_scan.atmosphere.density == AtmosphereDensity::Thin
                    && planet_scan.atmosphere.kind == *atmosphere_type
            }
            SpawnCondition::MinGravity(min_gravity) => planet_scan.surface_gravity >= *min_gravity,
            SpawnCondition::MaxGravity(max_gravity) => planet_scan.surface_gravity <= *max_gravity,
            SpawnCondition::PlanetClass(planet_class) => planet_scan.planet_class == *planet_class,
            SpawnCondition::ParentStarClass(star_class) => {
                true // FIXME: We need a way to get the parent star
            }
            SpawnCondition::ParentStarLuminosity(star_luminosity) => {
                true // FIXME: We need a way to get the parent star
            }
            SpawnCondition::MinOrEqualParentStarLuminosity(star_luminosity) => {
                true // FIXME: We need a way to get the parent star
            }
            SpawnCondition::SystemContainsPlanetClass(planet_class) => {
                true // FIXME: We need a way to get the parent star system
            }
            SpawnCondition::VolcanismType(volcanism_type) => {
                planet_scan.volcanism.kind == *volcanism_type
            }
            SpawnCondition::MinDistanceFromParentSun(min_distance) => {
                planet_scan.orbit_info.semi_major_axis >= *min_distance // FIXME: Double check units, are they both in AU/ls/Mm/m?
            }
            SpawnCondition::AnyVolcanism => planet_scan.volcanism.kind == VolcanismType::None,
            SpawnCondition::WithinNebulaRange(nebula_range) => {
                true // FIXME: We need a way to get the parent star system
            }
            SpawnCondition::GeologicalSignalsPresent => {
                true // FIXME: This information is only available after mapping the planet
            }
            SpawnCondition::Any(conditions) => conditions
                .iter()
                .any(|condition| condition.check_for_planet(planet_scan)),
            SpawnCondition::All(conditions) => conditions
                .iter()
                .all(|condition| condition.check_for_planet(planet_scan)),
        }
    }
}
