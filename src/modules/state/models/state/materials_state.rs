use crate::logs::LogEvent;
use crate::state::{StateContainer};
use crate::state::resolvers::materials_state_resolver::MaterialsStateResolver;

pub type MaterialsState = StateContainer<MaterialsStateResolver, LogEvent>;