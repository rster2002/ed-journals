use crate::logs::content::log_event_content::scan_event::ScanEvent;

#[derive(Debug)]
pub struct BodyState {
    pub scan: ScanEvent,
}
