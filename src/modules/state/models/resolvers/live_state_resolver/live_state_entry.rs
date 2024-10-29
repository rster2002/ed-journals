use serde::Serialize;
use crate::state::LiveState;

#[derive(Serialize)]
pub struct LiveStateEntry<'a> {
    pub name: &'a str,
    pub state: &'a LiveState,
}