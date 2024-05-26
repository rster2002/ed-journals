use serde::Serialize;

use crate::logs::scan_organic_event::{
    ScanOrganicEvent, ScanOrganicEventScanType,
};

#[derive(Debug, Serialize)]
pub struct OrganicState {
    pub events: Vec<Option<ScanOrganicEvent>>,
}

impl Default for OrganicState {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganicState {
    pub fn new() -> Self {
        OrganicState { events: Vec::new() }
    }

    pub fn push_event(&mut self, event: &ScanOrganicEvent) {
        self.events.push(Some(event.clone()));
    }

    pub fn clear_process(&mut self) {
        if self.get_progress().is_some() {
            self.events.push(None);
        }
    }

    pub fn get_progress(&self) -> Option<&ScanOrganicEventScanType> {
        Some(&self.events.iter().last()?.as_ref()?.scan_type)
    }

    pub fn completed(&self) -> bool {
        let Some(process) = self.get_progress() else {
            return false;
        };

        process == &ScanOrganicEventScanType::Log
    }
}
