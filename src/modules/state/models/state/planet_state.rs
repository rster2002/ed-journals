use crate::exobiology::TargetPlanet;
use crate::logs::scan_event::{ScanEvent, ScanEventKind, ScanEventPlanet};
use crate::logs::LogEvent;
use crate::state::models::resolvers::planet_state_resolver::{
    PlanetStateError, PlanetStateResolver,
};
use crate::state::models::state_container::StateContainer;
use std::collections::HashSet;
use crate::galaxy::LocalDistance;

pub type PlanetState = StateContainer<PlanetStateResolver, LogEvent>;

impl From<(&ScanEvent, &ScanEventPlanet)> for PlanetState {
    fn from(value: (&ScanEvent, &ScanEventPlanet)) -> Self {
        StateContainer::from(PlanetStateResolver {
            scan: value.0.clone(),
            saa_scan: None,
            saa_signals: Vec::new(),
            saa_genuses: None,
            scanned_species: HashSet::new(),
            touchdowns: Vec::new(),
            signal_counts: None,
            logged_species: HashSet::new(),
            commodity_signals: Vec::new(),
            exobiology_body: TargetPlanet {
                is_landable: value.1.landable,
                pressure: value.1.surface_pressure,
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
                semi_major_axis: LocalDistance::from_m(value.1.orbit_info.semi_major_axis),
                geological_signals_present: false,
            },
        })
    }
}

impl TryFrom<&ScanEvent> for PlanetState {
    type Error = PlanetStateError;

    fn try_from(value: &ScanEvent) -> Result<Self, Self::Error> {
        let ScanEventKind::Planet(planet) = &value.kind else {
            return Err(PlanetStateError::NotAPlanetScan);
        };

        Ok(PlanetState::from((value, planet)))
    }
}
