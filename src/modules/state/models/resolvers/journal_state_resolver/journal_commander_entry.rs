use std::cmp::Ordering;
use serde::Serialize;
use crate::galaxy::planet_distance;
use crate::state::{LiveState, LogState, OrganicLocation};

#[derive(Default, Serialize)]
pub struct JournalCommanderEntry {
    pub log_state: LogState,
    pub live_state: LiveState,
}

impl JournalCommanderEntry {
    /// Returns the distance between the player and the previously scanned organic, returning the
    /// distance in meters for the scan that is closest to the player.
    pub fn current_organic_distance(&self) -> Option<f32> {
        let current_organic = self.log_state
            .current_organic_progress
            .as_ref()?;

        let planet_status = self.live_state
            .current_planet_status()?;

        self.live_state
            .organic_locations
            .iter()
            .filter(|location| {
                location.body_name == planet_status.body_name
                    && location.species == current_organic.species
            })
            .map(|location| {
                planet_distance(
                    planet_status.planet_radius,
                    &location.coordinates,
                    &(planet_status.latitude, planet_status.longitude)
                )
            })
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }
}
