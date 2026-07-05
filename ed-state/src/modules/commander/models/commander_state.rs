use crate::commander::models::current_organic_progress::CurrentOrganicProgress;
use crate::commander::models::materials_state::MaterialsState;
use crate::modules::state::{EventSink, SinkResult};
use crate::modules::system::SystemState;
use crate::system::PlanetOrganic;
use ed_journals::galaxy::planet_distance;
use ed_journals::logs::commander_event::CommanderEvent;
use ed_journals::logs::scan_organic_event::ScanOrganicEventScanType;
use ed_journals::logs::touchdown_event::TouchdownEvent;
use ed_journals::logs::{LogEvent, LogEventContent};
use ed_journals::status::{PlanetStatus, Status, StatusContents};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct CommanderState {
    pub commander: Option<CommanderEvent>,

    /// Systems that the player has visited.
    pub systems: HashMap<u64, SystemState>,

    /// The system address the player is currently in.
    pub current_system: Option<u64>,

    /// Current touchdown event. Is set back to none when the player lifts off or dies.
    pub touchdown: Option<TouchdownEvent>,

    /// Keeps track of the current materials the player has.
    pub material_state: MaterialsState,

    /// The current organic progress.
    pub current_organic_progress: Option<CurrentOrganicProgress>,

    /// The lastest status captured.
    pub status: Option<StatusContents>,
}

impl CommanderState {
    pub fn current_system_state(&self) -> Option<&SystemState> {
        self.current_system
            .as_ref()
            .and_then(|id| self.systems.get(id))
    }

    /// Returns the current status of the commander on a planet.
    pub fn current_planet_status(&self) -> Option<&PlanetStatus> {
        self.status
            .as_ref()
            .and_then(|status| status.planet_status.as_ref())
    }

    /// Return the current planet coordinates if on a planet.
    pub fn planet_location(&self) -> Option<(f32, f32)> {
        let planet_status = self.status.as_ref()?.planet_status.as_ref()?;

        Some((planet_status.latitude, planet_status.longitude))
    }

    /// Returns current active organic for the commander.
    pub fn current_organic(&self) -> Option<&PlanetOrganic> {
        self.current_organic_progress.as_ref().and_then(|current| {
            self.systems
                .get(&current.system_address)
                .and_then(|system| system.planet_state.get(&current.body_id))
                .and_then(|planet| planet.organics.get(&current.genus))
        })
    }

    /// Return the current organic only if the commander is currently on the same planet.
    pub fn current_body_organic(&self) -> Option<&PlanetOrganic> {
        let current_system = self.current_system_state()?;
        let near_body = current_system.near_body?;
        let planet_state = current_system.planet_state.get(&near_body)?;
        let current_progress = self.current_organic_progress.as_ref()?;

        // Only calculate in the correct system and body.
        if current_progress.system_address != current_system.system_address
            || current_progress.body_id != planet_state.body_id
        {
            return None;
        }

        self.current_organic()
    }

    pub fn distance_from(&self, coordinates: &(f32, f32)) -> Option<f32> {
        let planet_status = self.current_planet_status()?;

        Some(planet_distance(
            planet_status.planet_radius,
            &(planet_status.latitude, planet_status.longitude),
            coordinates,
        ))
    }

    pub fn distance_from_first_organic(&self) -> Option<f32> {
        self.distance_from(&self.current_body_organic()?.first_scan.as_ref()?.location?)
    }

    pub fn distance_from_second_organic(&self) -> Option<f32> {
        self.distance_from(
            &self
                .current_body_organic()?
                .second_scan
                .as_ref()?
                .location?,
        )
    }

    /// Returns the current distance to the closest active organic.
    pub fn distance_from_nearest_organic(&self) -> Option<f32> {
        let first_distance = self.distance_from_first_organic()?;

        let Some(second_distance) = self.distance_from_second_organic() else {
            return Some(first_distance);
        };

        Some(first_distance.min(second_distance))
    }
}

impl EventSink for CommanderState {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        let mut result = SinkResult::default();

        if let Some(system_address) = log_event.content.system_address() {
            result.or_replace(
                self.systems
                    .entry(system_address)
                    .or_insert_with(|| SystemState::from(system_address))
                    .sink_log(log_event),
            );
        }

        result.or_replace(self.material_state.sink_log(log_event));

        match &log_event.content {
            LogEventContent::Commander(event) => {
                self.commander = Some(event.clone());
                result.accept();
            }
            LogEventContent::Location(location) => {
                self.current_system = Some(location.location_info.system_address);
                result.accept();
            }
            LogEventContent::FSDJump(fsd_jump) => {
                self.current_system = Some(fsd_jump.system_info.system_address);
                result.accept();
            }
            LogEventContent::ScanOrganic(scan_organic) => match &scan_organic.scan_type {
                ScanOrganicEventScanType::Log => {
                    self.current_organic_progress = Some(scan_organic.into());
                }
                ScanOrganicEventScanType::Analyse => {
                    self.current_organic_progress = None;
                }
                _ => {}
            },
            LogEventContent::Touchdown(event) => {
                self.touchdown = Some(event.clone());
                result.accept();
            }
            LogEventContent::Liftoff(_) => {
                self.touchdown = None;
                result.accept();
            }
            _ => {}
        }

        result
    }

    fn sink_status(&mut self, status: &Status) -> SinkResult {
        self.status = status.contents.clone();

        self.material_state.sink_status(status);

        for system in self.systems.values_mut() {
            system.sink_status(status);
        }

        SinkResult::Accepted
    }
}
