use crate::journal::JournalEvent;
use crate::state::models::resolvers::live_state_resolver::LiveStateResolver;
use crate::state::models::state_container::StateContainer;

pub type LiveState = StateContainer<LiveStateResolver, JournalEvent>;
