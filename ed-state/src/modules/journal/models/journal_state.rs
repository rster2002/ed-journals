use std::collections::HashMap;
use ed_journals::logs::{LogEvent, LogEventContent};
use crate::modules::state::{LogSink, SinkResult};

#[derive(Debug, Clone)]
pub struct JournalState<T>
where T : LogSink + Default
{
    pub commanders: HashMap<String, T>,
    current_commander_fid: Option<String>,
}

impl<T> LogSink for JournalState<T>
where T : LogSink + Default
{
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        if let LogEventContent::Commander(commander) = &log_event.content {
            self.current_commander_fid = Some(commander.fid.clone());
            self.commanders.entry(commander.fid.clone()).or_insert(T::default());
        }

        let Some(inner) = self.current_as_mut() else {
            return SinkResult::Ignored;
        };

        inner.sink_log(log_event)
    }
}

impl<T> JournalState<T>
where T : LogSink + Default
{
    /// Returns FID of the commander that last emitted the 'commander' event.
    pub fn current_fid(&self) -> Option<&String> {
        self.current_commander_fid.as_ref()
    }

    /// Returns a reference to the inner state of the currently processing commander.
    pub fn current_as_ref(&self) -> Option<&T> {
        self.current_commander_fid
            .as_ref()
            .and_then(|fid| self.commanders.get(fid))
    }

    /// Returns a mutable reference to the inner state of the current processing commander.
    pub fn current_as_mut(&mut self) -> Option<&mut T> {
        self.current_commander_fid
            .as_ref()
            .and_then(|fid| self.commanders.get_mut(fid))
    }
}