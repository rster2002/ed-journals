use crate::modules::galaxy::{Atmosphere, PlanetClass, StarClass};

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
