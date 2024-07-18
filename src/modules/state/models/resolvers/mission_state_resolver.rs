use std::collections::HashMap;
use serde::Serialize;
use crate::logs::LogEvent;
use crate::state::models::feed_result::FeedResult;
use crate::state::traits::state_resolver::StateResolver;

#[derive(Serialize, Default)]
pub struct MissionStateResolver {
    pub missions: HashMap<u64, Mission>,
}

#[derive(Serialize)]
pub struct Mission {

}

impl StateResolver<LogEvent> for MissionStateResolver {
    fn feed(&mut self, input: &LogEvent) -> FeedResult {
        FeedResult::Accepted
    }
}
