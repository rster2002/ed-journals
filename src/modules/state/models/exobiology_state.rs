use std::collections::{HashMap, HashSet};

use crate::{
    logs::content::log_event_content::{
        fsd_jump_event::FSDJumpEvent, fss_body_signals_event::FSSBodySignalsEvent,
        location_event::LocationEvent, scan_event::ScanEvent,
    },
    models::exploration::species::Species,
    modules::exobiology::models::spawn_source::SpawnSource,
};

#[derive(Debug)]
pub struct ExobiologyState {
    pub spawn_sources: HashMap<String, SpawnSource>,
    scan_events: Vec<ScanEvent>,
    fss_body_signals_events: Vec<FSSBodySignalsEvent>,
    location_events: Vec<LocationEvent>,
    fsd_jump_events: Vec<FSDJumpEvent>,
}

impl ExobiologyState {
    pub fn new() -> Self {
        ExobiologyState {
            spawn_sources: HashMap::new(),
            scan_events: Vec::new(),
            fss_body_signals_events: Vec::new(),
            location_events: Vec::new(),
            fsd_jump_events: Vec::new(),
        }
    }

    /// Adds a scan event to a SpawnSource's pool of information.
    pub fn feed_scan_event(&mut self, event: &ScanEvent) {
        self.scan_events.push(event.clone());
        self.add_spawn_source(event.body_name.clone());
        self.recalculate_spawnable_species();
    }

    /// Adds a FSS body signals event to a SpawnSource's pool of information.
    pub fn feed_fss_body_signals_event(&mut self, event: &FSSBodySignalsEvent) {
        self.fss_body_signals_events.push(event.clone());
        self.add_spawn_source(event.body_name.clone());
        self.recalculate_spawnable_species();
    }

    /// Adds a location event to a SpawnSource's pool of information.
    pub fn feed_location_event(&mut self, event: &LocationEvent) {
        self.location_events.push(event.clone());
        self.recalculate_spawnable_species();
    }

    /// Adds a FSD jump event to a SpawnSource's pool of information.
    pub fn feed_fsd_jump_event(&mut self, event: &FSDJumpEvent) {
        self.fsd_jump_events.push(event.clone());
        self.recalculate_spawnable_species();
    }

    /// Returns a set of species that can be spawned on the given body.
    /// This set is calculated based on information provided through the `feed_scan_event` and
    /// `feed_fss_body_signals_event` functions.
    /// If the body is not known to the ExobiologyState, an empty set is returned.
    pub fn get_spawnable_species(&self, body_name: String) -> HashSet<Species> {
        if let Some(spawn_source) = self.spawn_sources.get(&body_name) {
            spawn_source.get_spawnable_species()
        } else {
            HashSet::new()
        }
    }

    fn add_spawn_source(&mut self, body_name: String) {
        if !self.spawn_sources.contains_key(&body_name) {
            self.spawn_sources
                .insert(body_name.clone(), SpawnSource::new(body_name.clone()));
        }
    }

    fn recalculate_spawnable_species(&mut self) {
        for spawn_source in self.spawn_sources.values_mut() {
            // TODO: Maybe we should figure out if this spawn source *needs* to be recalculated
            //  aka; if it has new events to process. For now I suppose it's fine, as the `feed_***_event`
            //  functions do a quick return if the event is irrelevant.
            for scan in &self.scan_events {
                spawn_source.feed_scan_event(scan);
            }

            for fss_body_signals in &self.fss_body_signals_events {
                spawn_source.feed_fss_body_signals_event(fss_body_signals);
            }

            for location in &self.location_events {
                spawn_source.feed_location_event(location);
            }

            for fsd_jump in &self.fsd_jump_events {
                spawn_source.feed_fsd_jump_event(fsd_jump);
            }
        }
    }
}

impl Default for ExobiologyState {
    fn default() -> Self {
        ExobiologyState::new()
    }
}
