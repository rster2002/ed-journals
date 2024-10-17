use crate::logs::LogEvent;
use crate::state::models::feed_result::FeedResult;
use crate::state::traits::state_resolver::StateResolver;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Default)]
pub struct MissionStateResolver {
    pub missions: HashMap<u64, Mission>,
}

#[derive(Serialize)]
pub struct Mission {}

impl StateResolver<LogEvent> for MissionStateResolver {
    fn feed(&mut self, _input: &LogEvent) -> FeedResult {
        FeedResult::Accepted
    }
}
