use crate::journal::JournalEvent;
use crate::state::models::resolvers::journal_state_resolver::JournalStateResolver;
use crate::state::StateContainer;

pub type JournalState = StateContainer<JournalStateResolver, JournalEvent>;
