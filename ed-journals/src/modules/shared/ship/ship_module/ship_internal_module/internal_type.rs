use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum InternalType {
    Core,
    Optional,
}
