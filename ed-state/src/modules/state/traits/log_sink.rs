use ed_journals::logs::LogEvent;
use crate::modules::state::models::sink_result::SinkResult;

pub trait LogSink {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult;
}