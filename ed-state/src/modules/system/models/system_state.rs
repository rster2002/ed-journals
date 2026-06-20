use crate::modules::state::{EventSink, SinkResult};
use crate::modules::system::PlanetState;
use chrono::{DateTime, Utc};
use ed_journals::civilization::LocationInfo;
use ed_journals::logs::fss_signal_discovered_event::FSSSignalDiscoveredEvent;
use ed_journals::logs::scan_event::{ScanEvent, ScanEventKind};
use ed_journals::logs::{LogEvent, LogEventContent};
use ed_journals::partials::PartialSystemInfo;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct SystemState {
    /// The address of the system.
    pub partial_system_info: PartialSystemInfo,

    /// Information about the system.
    pub location_info: Option<LocationInfo>,

    /// Entries for state for planets in the system.
    pub planet_state: HashMap<u8, PlanetState>,

    /// Scans for each star in the system.
    pub star_scans: HashMap<u8, ScanEvent>,

    /// Scans for each cluster in the system.
    pub belt_scans: HashMap<u8, ScanEvent>,

    /// Times when the player was in the system.
    pub visits: Vec<DateTime<Utc>>,

    /// Times when the player's was in the system.
    pub carrier_visits: Vec<DateTime<Utc>>,

    /// The number of bodies that are present in the system.
    pub number_of_bodies: Option<u8>,

    /// Current progress of discovering all bodies in the system.
    pub progress: f32,

    /// Whether all bodies have been discovered in the system.
    pub all_found: bool,

    /// List of station signals.
    pub station_signals: Vec<FSSSignalDiscoveredEvent>,
}

impl SystemState {
    /// Returns the total number of scans, which includes planets, stars, and belt clusters.
    pub fn nr_of_scans(&self) -> usize {
        self.planet_state.len() + self.star_scans.len() + self.belt_scans.len()
    }

    /// Returns the total number of scanned bodies, which includes planets and stars. Take note
    /// that this does not include scanned belt clusters as they are not counted towards the total
    /// number of scanned bodies in game.
    pub fn nr_of_scanned_bodies(&self) -> usize {
        self.planet_state.len() + self.star_scans.len()
    }

    /// Returns all the scan events for this system.
    pub fn all_scans(&self) -> Vec<&ScanEvent> {
        let mut result = Vec::with_capacity(self.nr_of_scans());

        for planet in self.planet_state.values() {
            if let Some(scan) = &planet.scan {
                result.push(scan);
            }
        }

        for star_scan in self.star_scans.values() {
            result.push(star_scan)
        }

        for belt_scan in self.belt_scans.values() {
            result.push(belt_scan)
        }

        result
    }
}

impl From<PartialSystemInfo> for SystemState {
    fn from(value: PartialSystemInfo) -> Self {
        SystemState {
            partial_system_info: value,
            ..Default::default()
        }
    }
}

impl EventSink for SystemState {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        let mut result = SinkResult::Ignored;

        if log_event
            .content
            .system_address()
            .is_none_or(|address| address != self.partial_system_info.system_address)
        {
            return result;
        }

        if let Some(location_info) = log_event.content.location_info()
            && self.location_info.is_none()
        {
            self.location_info = Some(location_info.clone());
        }

        match &log_event.content {
            LogEventContent::FSSDiscoveryScan(event) => {
                self.number_of_bodies = Some(event.body_count);
                self.progress = event.progress;
                result.accept();
            }
            LogEventContent::FSSAllBodiesFound(event) => {
                self.number_of_bodies = Some(event.count);
                self.all_found = true;
                result.accept();
            }
            LogEventContent::FSSSignalDiscovered(event) if event.is_station => {
                self.station_signals.push(event.clone());
                result.accept();
            }
            LogEventContent::Scan(event) => {
                match &event.kind {
                    ScanEventKind::Star(_star) => {
                        self.star_scans.insert(event.body_id, event.clone());
                        result.accept();
                    }
                    ScanEventKind::BeltCluster(_) => {
                        self.belt_scans.insert(event.body_id, event.clone());
                        result.accept();
                    }
                    _ => {}
                }

                if let Some(total_bodies) = self.number_of_bodies {
                    let new_factor = self.nr_of_scanned_bodies() as f32 / total_bodies as f32;

                    if new_factor > self.progress {
                        self.progress = new_factor;
                        result.accept();
                    }
                }
            }

            _ => {}
        }

        if let Some(body_id) = log_event.content.body_id() {
            result.or_replace(
                self.planet_state
                    .entry(body_id)
                    .or_insert_with(|| PlanetState::from(body_id))
                    .sink_log(log_event),
            )
        }

        result
    }
}
