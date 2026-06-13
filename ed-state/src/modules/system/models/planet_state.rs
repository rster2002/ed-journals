use ed_journals::logs::{LogEvent, LogEventContent};
use ed_journals::logs::scan_event::{ScanEvent, ScanEventKind};
use crate::modules::state::{EventSink, SinkResult};

#[derive(Debug, Default, Clone)]
pub struct PlanetState {
    pub body_id: u8,
    pub scan: Option<ScanEvent>,
}

impl From<u8> for PlanetState {
    fn from(value: u8) -> Self {
        PlanetState {
            body_id: value,
            ..Default::default()
        }
    }
}

impl EventSink for PlanetState {
    fn sink_log(&mut self, log_event: &LogEvent) -> SinkResult {
        let mut result = SinkResult::Ignored;

        if !log_event.content.body_id().is_some_and(|body_id| body_id == self.body_id) {
            return result;
        }
        
        match &log_event.content {
            LogEventContent::Scan(event) => {
                if event.kind.is_planet() {
                    self.scan = Some(event.clone());
                    result.accept();
                }
            }
            _ => {},
        }

        result
    }
}
