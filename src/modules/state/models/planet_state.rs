use std::collections::{HashMap, HashSet};

use crate::exobiology::{SpawnSource, TargetPlanet, TargetSystem};
use crate::exploration::PlanetarySignalType;
use serde::Serialize;
use thiserror::Error;

use crate::logs::content::log_event_content::fss_body_signals_event::FSSBodySignalEventSignal;
use crate::logs::content::log_event_content::saa_scan_complete_event::SAAScanCompleteEvent;
use crate::logs::content::log_event_content::saa_signals_found_event::SAASignalsFoundEventSignal;
use crate::logs::content::log_event_content::scan_event::{
    ScanEvent, ScanEventKind, ScanEventPlanet,
};
use crate::logs::content::log_event_content::touchdown_event::TouchdownEvent;
use crate::logs::content::{LogEvent, LogEventContent};
use crate::modules::exobiology::{Genus, Species};
use crate::state::models::feed_result::FeedResult;
use crate::state::models::organic_state::OrganicState;

#[derive(Debug, Serialize)]
pub struct PlanetState {
    pub scan: ScanEvent,

    // pub fss_signals: Vec<FSSBodySignalEventSignal>,

    // pub saa_scan: Option<SAAScanCompleteEvent>,
    // pub saa_signals: Vec<SAASignalsFoundEventSignal>,
    // pub saa_genuses: Vec<Genus>,

    pub touchdowns: Vec<TouchdownEvent>,
    // pub organics: HashMap<Species, OrganicState>,
    pub exobiology_body: TargetPlanet,
}

#[derive(Debug, Error)]
pub enum BodyStateError {
    #[error("The provided scan event is not that of a planet")]
    NotAPlanetScan,
}

impl PlanetState {
    // pub fn new(scan_event: ScanEvent) -> Option<Self> {
    //     BodyState {
    //         scan: scan_event,
    //         fss_signals: Vec::new(),
    //         saa_scan: None,
    //         saa_signals: Vec::new(),
    //         saa_genuses: Vec::new(),
    //         touchdowns: Vec::new(),
    //         organics: HashMap::new(),
    //         exobiology_body: TargetPlanet {
    //             atmosphere: scan_event.,
    //             gravity: Gravity(),
    //             class: PlanetClass::MetalRichBody,
    //             surface_temperature: 0.0,
    //             volcanism: Volcanism {},
    //             materials: Default::default(),
    //             composition: Default::default(),
    //             parents: vec![],
    //             semi_major_axis: 0.0,
    //             geological_signals_present: false,
    //         }
    //     }
    // }

    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        let Some(body_id) = log_event.content.body_id() else {
            return FeedResult::Skipped;
        };

        if body_id != self.scan.body_id {
            return FeedResult::Skipped;
        }

        match &log_event.content {
            // LogEventContent::SAAScanComplete(scan_complete) => {
            //     self.saa_scan = Some(scan_complete.clone());
            // }
            // LogEventContent::SAASignalsFound(signals) => {
            //     self.saa_signals.clone_from(&signals.signals);
            //     self.saa_genuses = signals
            //         .genuses
            //         .iter()
            //         .map(|signal| signal.genus.clone())
            //         .collect();
            // }
            LogEventContent::FSSBodySignals(body_signals) => {
                // self.fss_signals.clone_from(&body_signals.signals);

                self.exobiology_body.geological_signals_present = body_signals
                    .signals
                    .iter()
                    .find(|signal| signal.kind == PlanetarySignalType::Geological)
                    .is_some();
            }
            LogEventContent::Touchdown(touchdown) => {
                if touchdown.on_planet {
                    self.touchdowns.push(touchdown.clone());
                }
            }
            // LogEventContent::ScanOrganic(scanned_organic) => {
            //
            // },
            _ => {}
        }

        FeedResult::Accepted
    }

    // pub fn clear_organic_process(&mut self) {
    //     for organic in self.organics.values_mut() {
    //         organic.clear_process();
    //     }
    // }

    pub fn get_spawnable_species(&self, target_system: &TargetSystem) -> Vec<Species> {
        let spawn_source = SpawnSource {
            target_system,
            target_planet: &self.exobiology_body,
        };

        spawn_source.get_spawnable_species()
    }
}

impl From<(&ScanEvent, &ScanEventPlanet)> for PlanetState {
    fn from(value: (&ScanEvent, &ScanEventPlanet)) -> Self {
        PlanetState {
            scan: value.0.clone(),
            // fss_signals: vec![],
            // saa_scan: None,
            // saa_signals: vec![],
            // saa_genuses: vec![],
            touchdowns: vec![],
            // organics: Default::default(),
            exobiology_body: TargetPlanet {
                atmosphere: value.1.atmosphere.clone(),
                gravity: value.1.surface_gravity.clone(),
                class: value.1.planet_class.clone(),
                surface_temperature: value.1.surface_temperature,
                volcanism: value.1.volcanism.clone(),
                materials: HashSet::from_iter(
                    value
                        .1
                        .materials
                        .clone()
                        .into_iter()
                        .map(|entry| entry.name),
                ),
                composition: value.1.composition.clone(),
                parents: value.0.parents.clone(),
                semi_major_axis: value.1.orbit_info.semi_major_axis,
                geological_signals_present: false,
            },
        }
    }
}

impl TryFrom<&ScanEvent> for PlanetState {
    type Error = BodyStateError;

    fn try_from(value: &ScanEvent) -> Result<Self, Self::Error> {
        let ScanEventKind::Planet(planet) = &value.kind else {
            return Err(BodyStateError::NotAPlanetScan);
        };

        Ok(PlanetState::from((value, planet)))
    }
}
