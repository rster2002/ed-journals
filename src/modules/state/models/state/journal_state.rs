use crate::journal::JournalEvent;
use crate::state::models::resolvers::journal_state_resolver::JournalStateResolver;
use crate::state::{LiveState, StateContainer};
use std::collections::HashMap;

pub type JournalState = StateContainer<JournalStateResolver, JournalEvent>;

impl From<HashMap<String, LiveState>> for JournalState {
    fn from(value: HashMap<String, LiveState>) -> Self {
        StateContainer::from(JournalStateResolver::from(value))
    }
}
