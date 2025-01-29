use crate::logs::LogEvent;
use crate::state::resolvers::materials_state_resolver::MaterialsStateResolver;
use crate::state::StateContainer;

pub type MaterialsState = StateContainer<MaterialsStateResolver, LogEvent>;
