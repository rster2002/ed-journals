use crate::modules::state::{EventSink, SinkResult};
use ed_journals::logs::{LogEvent, LogEventContent};
use ed_journals::status::Status;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct JournalState<T>
where
    T: EventSink + Default,
{
    pub commanders: HashMap<String, T>,
    current_commander_fid: Option<String>,
}

impl<T> EventSink for JournalState<T>
where
    T: EventSink + Default,
{
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        if let LogEventContent::Commander(commander) = &log_event.content {
            self.current_commander_fid = Some(commander.fid.clone());
            self.commanders
                .entry(commander.fid.clone())
                .or_insert(T::default());
        }

        let Some(inner) = self.current_as_mut() else {
            return SinkResult::Ignored;
        };

        inner.sink_log(log_event)
    }

    fn sink_status(&mut self, status: &Status) -> SinkResult {
        self.current_as_mut()
            .and_then(|commander| Some(commander.sink_status(&status)))
            .unwrap_or(SinkResult::Ignored)
    }
}

impl<T> JournalState<T>
where
    T: EventSink + Default,
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
