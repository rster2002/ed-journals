use std::collections::HashMap;

use serde::Serialize;
use crate::exploration::calculate_estimated_worth;

use crate::logs::content::log_event_content::commander_event::CommanderEvent;
use crate::logs::content::log_event_content::scan_organic_event::ScanOrganicEventScanType;
use crate::logs::content::{LogEvent, LogEventContent};
use crate::logs::content::log_event_content::scan_event::ScanEvent;
use crate::modules::civilization::LocationInfo;
use current_organic::CurrentOrganic;
use crate::state::models::carrier_state::CarrierState;
use crate::state::models::feed_result::FeedResult;
use crate::state::models::materials_state::MaterialsState;
use crate::state::SystemState;
use crate::try_feed;

pub mod current_organic;

#[derive(Serialize)]
pub struct CommanderState {
    pub fid: String,
    pub name: String,
    pub systems: HashMap<u64, SystemState>,
    pub current_system: Option<u64>,
    pub current_organic: Option<CurrentOrganic>,
    pub current_organic_process: Option<ScanOrganicEventScanType>,
    pub current_exploration_data: Vec<ScanEvent>,
    pub material_state: MaterialsState,
    pub carrier_state: Option<CarrierState>,
}

impl CommanderState {
    pub fn feed_log_event(&mut self, log_event: &LogEvent) -> FeedResult {
        match &log_event.content {
            LogEventContent::Scan(event) => {
                self.current_exploration_data.push(event.clone());
            },
            LogEventContent::Died(event) => {
                self.current_exploration_data.clear();
            },
            LogEventContent::MultiSellExplorationData(event) => {
                for system in &event.discovered {
                    self.current_exploration_data
                        .retain(|item| {
                            item.star_system != system.system_name
                        });
                }
            },
            LogEventContent::SellExplorationData(event) => {
                for system in &event.systems {
                    self.current_exploration_data
                        .retain(|item| {
                            &item.star_system != system
                        });
                }
            },
            LogEventContent::Location(location) => {
                self.current_system = Some(location.location_info.system_address);

                let system = self.upset_system(&location.location_info);
                system.visit(&log_event.timestamp);
            }
            LogEventContent::CarrierJump(carrier_jump) => {
                let system = self.upset_system(&carrier_jump.system_info);
                system.carrier_visit(&log_event.timestamp);
            }
            LogEventContent::FSDJump(fsd_jump) => {
                self.current_system = Some(fsd_jump.system_info.system_address);

                let system = self.upset_system(&fsd_jump.system_info);
                system.visit(&log_event.timestamp);
            }
            LogEventContent::ScanOrganic(scan_organic) => match &scan_organic.scan_type {
                ScanOrganicEventScanType::Sample => {
                    self.current_organic_process = Some(ScanOrganicEventScanType::Sample);
                    self.current_organic = Some(CurrentOrganic {
                        system_address: scan_organic.system_address,
                        body_id: scan_organic.body,
                        species: scan_organic.species.clone(),
                    });
                },
                ScanOrganicEventScanType::Analyse => {
                    self.current_organic_process = Some(ScanOrganicEventScanType::Analyse);
                },
                ScanOrganicEventScanType::Log => {
                    self.current_organic = None;
                },
            },

            LogEventContent::Materials(event) => {
                for material in &event.raw {
                    self.material_state.set_material_count(material.name.clone(), material.count);
                }

                for material in &event.encoded {
                    self.material_state.set_material_count(material.name.clone(), material.count);
                }

                for material in &event.manufactured {
                    self.material_state.set_material_count(material.name.clone(), material.count);
                }
            }
            LogEventContent::MaterialCollected(event) => {
                self.material_state.add_material_count(event.name.clone(), event.count);
            },
            LogEventContent::MaterialDiscarded(event) => {
                self.material_state.remove_material_count(event.name.clone(), event.count);
            },
            LogEventContent::MaterialTrade(event) => {
                self.material_state.remove_material_count(event.paid.material.clone(), event.paid.quantity);
                self.material_state.add_material_count(event.received.material.clone(), event.received.quantity);
            },

            LogEventContent::CarrierStats(stats) => {
                if self.carrier_state.is_none() {
                    let mut state: CarrierState = stats.clone().into();
                    state.feed_log_event(&log_event);

                    self.carrier_state = Some(state);
                }
            },

            LogEventContent::CarrierJump(_)
            | LogEventContent::CarrierBuy(_)
            | LogEventContent::CarrierJumpRequest(_)
            | LogEventContent::CarrierDecommission(_)
            | LogEventContent::CarrierCancelDecommission(_)
            | LogEventContent::CarrierBankTransfer(_)
            | LogEventContent::CarrierDepositFuel(_)
            | LogEventContent::CarrierCrewServices(_)
            | LogEventContent::CarrierFinance(_)
            | LogEventContent::CarrierShipPack(_)
            | LogEventContent::CarrierModulePack(_)
            | LogEventContent::CarrierTradeOrder(_)
            | LogEventContent::CarrierDockingPermission(_)
            | LogEventContent::CarrierNameChange(_)
            | LogEventContent::CarrierJumpCancelled(_) => {
                match &mut self.carrier_state {
                    Some(state) => { try_feed!(state.feed_log_event(&log_event)); },
                    None => return FeedResult::Later,
                }
            },

            _ => {}
        }

        let carrier_has_been_scrapped = self.carrier_state
            .as_ref()
            .is_some_and(|state| state.has_been_scrapped(&log_event.timestamp));

        if carrier_has_been_scrapped {
            self.carrier_state = None;
        }

        if let Some(address) = log_event.content.system_address() {
            let Some(system) = self.systems.get_mut(&address) else {
                return FeedResult::Later;
            };

            try_feed!(system.feed_log_event(log_event));
        }

        FeedResult::Accepted
    }

    pub fn upset_system(&mut self, location_info: &LocationInfo) -> &mut SystemState {
        self.systems
            .entry(location_info.system_address)
            .or_insert_with(|| location_info.into());

        self.systems
            .get_mut(&location_info.system_address)
            .expect("Should have been added")
    }

    pub fn current_system(&self) -> Option<&SystemState> {
        self.systems.get(&self.current_system?)
    }

    pub fn system_by_address(&self, address: u64) -> Option<&SystemState> {
        self.systems.get(&address)
    }

    pub fn current_exploration_worth(&self) -> u64 {
        self.current_exploration_data
            .iter()
            .map(|item| calculate_estimated_worth(item))
            .sum()
    }
}

impl From<&CommanderEvent> for CommanderState {
    fn from(value: &CommanderEvent) -> Self {
        CommanderState {
            fid: value.fid.to_string(),
            name: value.name.to_string(),
            systems: HashMap::new(),
            current_system: None,
            current_organic: None,
            current_organic_process: None,
            current_exploration_data: Vec::new(),
            material_state: MaterialsState::default(),
            carrier_state: None,
        }
    }
}
