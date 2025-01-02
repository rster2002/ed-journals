//! High level resolver for resolving all log events. This does not however differentiate between
//! multiple commanders, whereas [GameStateResolver](super::game_state_resolver::GameStateResolver)
//! does.

use std::collections::HashMap;

use serde::Serialize;

use crate::civilization::LocationInfo;
use crate::exploration::calculate_estimated_worth;
use crate::logs::loadout_event::LoadoutEvent;
use crate::logs::rank_event::RankEvent;
use crate::logs::reputation_event::ReputationEvent;
use crate::logs::scan_event::ScanEvent;
use crate::logs::scan_organic_event::ScanOrganicEventScanType;
use crate::logs::statistics_event::StatisticsEvent;
use crate::logs::{LogEvent, LogEventContent};
use crate::state::models::feed_result::FeedResult;
use crate::state::models::state::carrier_state::CarrierState;
use crate::state::traits::state_resolver::StateResolver;
use crate::state::SystemState;
use crate::state::{MaterialsState, ShipyardState};
use current_organic_progress::CurrentOrganicProgress;

pub mod current_organic_progress;

/// High level resolver for resolving all log events. This does not however differentiate between
/// multiple commanders, whereas [GameStateResolver](super::game_state_resolver::GameStateResolver)
/// does.
#[derive(Serialize, Default)]
pub struct LogStateResolver {
    /// Systems that the player has visited.
    pub systems: HashMap<u64, SystemState>,

    /// The system address the player is currently in.
    pub current_system: Option<u64>,

    /// Information about the current exobiology scans the player is performing.
    pub current_organic_progress: Option<CurrentOrganicProgress>,

    /// List of scans that have been performed since the last time the player has sold data or died.
    pub current_exploration_data: Vec<ScanEvent>,

    /// Keeps track of the current materials the player has.
    pub material_state: MaterialsState,
    // pub mission_state: MissionState,
    /// Information about the player's stored ships.
    pub shipyard_state: ShipyardState,

    /// State regarding the player's carrier if they own one. `None` means that the player does
    /// not own one or that their carrier has been scrapped.
    pub carrier_state: Option<CarrierState>,

    /// The lastest loadout event fired for the player.
    pub current_loadout: Option<LoadoutEvent>,

    /// The lastest rank event fired for the player.
    pub rank: Option<RankEvent>,

    /// The lastest reputation event fired for the player.
    pub reputation: Option<ReputationEvent>,

    /// The lastest statistics event fired for the player.
    pub statistics: Option<StatisticsEvent>,
}

impl StateResolver<LogEvent> for LogStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        match &input.content {
            LogEventContent::Scan(event) => {
                self.current_exploration_data.push(event.clone());
            }
            LogEventContent::Died(_) => {
                self.current_exploration_data.clear();
            }
            LogEventContent::Rank(ranks) => {
                self.rank = Some(ranks.clone());
            }
            LogEventContent::Reputation(reputation) => {
                self.reputation = Some(reputation.clone());
            }
            LogEventContent::Statistics(statistics) => {
                self.statistics = Some(statistics.clone());
            }
            LogEventContent::MultiSellExplorationData(event) => {
                for system in &event.discovered {
                    self.current_exploration_data
                        .retain(|item| item.star_system != system.system_name);
                }
            }
            LogEventContent::SellExplorationData(event) => {
                for system in &event.systems {
                    self.current_exploration_data
                        .retain(|item| &item.star_system != system);
                }
            }
            LogEventContent::Location(location) => {
                self.current_system = Some(location.location_info.system_address);

                let system = self.upsert_system(&location.location_info);
                system.visit(&input.timestamp);
            }
            LogEventContent::FSDJump(fsd_jump) => {
                self.current_system = Some(fsd_jump.system_info.system_address);

                let system = self.upsert_system(&fsd_jump.system_info);
                system.visit(&input.timestamp);
            }
            LogEventContent::ScanOrganic(scan_organic) => match &scan_organic.scan_type {
                ScanOrganicEventScanType::Log => {
                    self.current_organic_progress = Some(scan_organic.into());
                }
                ScanOrganicEventScanType::Sample => {
                    if let Some(progress) = self.current_organic_progress.as_mut() {
                        if progress.second_scan.is_none() {
                            progress.second_scan = Some(scan_organic.clone());
                        }
                    }
                }
                ScanOrganicEventScanType::Analyse => {
                    self.current_organic_progress = None;
                }
            },

            LogEventContent::CarrierStats(stats) => {
                if self.carrier_state.is_none() {
                    let mut state: CarrierState = stats.clone().into();
                    state.feed(input);

                    self.carrier_state = Some(state);
                }
            }

            LogEventContent::Loadout(event) => {
                self.current_loadout = Some(event.clone());
            }

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
                if let LogEventContent::CarrierJump(carrier_jump) = &input.content {
                    let system = self.upsert_system(&carrier_jump.system_info);
                    system.carrier_visit(&input.timestamp);
                }

                match &mut self.carrier_state {
                    Some(state) => state.feed(input),
                    None => return FeedResult::Later,
                }
            }
            _ => {}
        }

        let carrier_has_been_scrapped = self
            .carrier_state
            .as_ref()
            .is_some_and(|state| state.has_been_scrapped(&input.timestamp));

        if carrier_has_been_scrapped {
            self.carrier_state = None;
        }

        if let Some(address) = input.content.system_address() {
            let Some(system) = self.systems.get_mut(&address) else {
                return FeedResult::Later;
            };

            system.feed(input);
        }

        self.material_state.feed(input);
        self.shipyard_state.feed(input);

        FeedResult::Accepted
    }

    fn flush_inner(&mut self) {
        if let Some(carrier_state) = &mut self.carrier_state {
            carrier_state.flush();
        }
    }
}

impl LogStateResolver {
    pub fn upsert_system(&mut self, location_info: &LocationInfo) -> &mut SystemState {
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
            .map(calculate_estimated_worth)
            .sum()
    }
}
