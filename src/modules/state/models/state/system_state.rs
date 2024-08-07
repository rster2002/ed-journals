use std::collections::HashMap;
use crate::civilization::LocationInfo;
use crate::exobiology::TargetSystem;
use crate::logs::LogEvent;
use crate::state::models::resolvers::system_state_resolver::SystemStateResolver;
use crate::state::models::state_container::StateContainer;

pub type SystemState = StateContainer<SystemStateResolver, LogEvent>;

impl From<&LocationInfo> for SystemState {
    fn from(value: &LocationInfo) -> Self {
        StateContainer::from(SystemStateResolver {
            location_info: value.clone(),
            planet_state: HashMap::new(),
            star_scans: HashMap::new(),
            belt_scans: HashMap::new(),
            visits: Vec::new(),
            carrier_visits: Vec::new(),
            number_of_bodies: None,
            progress: 0.0,
            all_found: false,
            station_signals: Vec::new(),
            exobiology_system: TargetSystem {
                star_system_position: value.star_pos,
                planet_classes_in_system: Default::default(),
                stars_in_system: Default::default(),
            },
        })
    }
}
