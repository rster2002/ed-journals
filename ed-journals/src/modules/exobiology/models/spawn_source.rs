use crate::shared::galaxy::atmosphere::Atmosphere;
use crate::shared::galaxy::planet_class::PlanetClass;
use crate::shared::galaxy::star_class::StarClass;

#[derive(Debug)]
pub struct SpawnSource {
    pub target_planet: TargetPlanet,
    pub planet_types: Vec<PlanetClass>,
}

#[derive(Debug)]
pub struct TargetPlanet {
    pub atmosphere: Atmosphere,
    pub gravity: f32,
    pub surface_temperature: f32,
    pub parent_star_class: StarClass,
}
