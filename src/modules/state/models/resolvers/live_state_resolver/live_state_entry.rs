use crate::state::LiveState;
use serde::Serialize;

#[derive(Serialize)]
pub struct LiveStateEntry<'a> {
    pub name: &'a str,
    pub state: &'a LiveState,
}
