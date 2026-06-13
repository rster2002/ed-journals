use crate::modules::state::models::sink_result::SinkResult;
use ed_journals::logs::LogEvent;
use ed_journals::status::Status;

/// Collects events and state from different sources and processes it.
pub trait EventSink {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        SinkResult::Ignored
    }

    fn sink_status(&mut self, status: &Status) -> SinkResult {
        SinkResult::Ignored
    }
}
