use crate::{
    logs::content::log_event_content::scan_event::ScanEventPlanet,
    models::exploration::species::Species,
};
use strum::IntoEnumIterator;

pub mod models;

/// Returns a list of species that can spawn on the given planet.
pub fn get_spawnable_species(planet_scan: &ScanEventPlanet) -> Vec<Species> {
    Species::iter()
        .filter(|species| check_species_can_spawn(planet_scan, species))
        .collect()
}

fn check_species_can_spawn(planet_scan: &ScanEventPlanet, species: &Species) -> bool {
    species
        .spawn_conditions()
        .iter()
        .all(|condition| condition.check_for_planet(planet_scan))
}
