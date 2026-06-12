/// Indicates what the state has done with the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SinkResult {
    /// The input was accepted and resolved in the state; something has changed in the state.
    Accepted,

    /// The state can't/won't process the input.
    Ignored,
}

impl SinkResult {
    pub fn is_accepted(&self) -> bool {
        matches!(self, SinkResult::Accepted)
    }

    pub fn is_ignored(&self) -> bool {
        matches!(self, SinkResult::Ignored)
    }
}