use crate::logs::carrier_stats_event::CarrierStatsEvent;
use crate::logs::LogEvent;
use crate::state::models::resolvers::carrier_state_resolver::CarrierStateResolver;
use crate::state::models::state_container::StateContainer;

pub type CarrierState = StateContainer<CarrierStateResolver, LogEvent>;

impl From<CarrierStatsEvent> for CarrierState {
    fn from(value: CarrierStatsEvent) -> Self {
        StateContainer::from(CarrierStateResolver {
            stats: value,
            flight_history: Vec::new(),
            last_location_update: Default::default(),
            scheduled_jump: None,
            scrap_time: None,
        })
    }
}
