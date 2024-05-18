use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum HardpointType {
    FullSized,
    Utility,
}
