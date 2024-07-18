use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SignalCounts {
    pub human_signal_count: usize,
    pub biological_signal_count: usize,
    pub geological_signal_count: usize,
    pub thargoid_signal_count: usize,
    pub guardian_signal_count: usize,
    pub other_signal_count: usize,
}
