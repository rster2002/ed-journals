/// Indicates what the state has done with the input.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum SinkResult {
    /// The state can't/won't process the input.
    #[default]
    Ignored,

    /// The input was accepted and resolved in the state; something has changed in the state.
    Accepted,
}

impl SinkResult {
    pub fn is_accepted(&self) -> bool {
        matches!(self, SinkResult::Accepted)
    }

    pub fn is_ignored(&self) -> bool {
        matches!(self, SinkResult::Ignored)
    }

    pub fn or_replace(&mut self, other: SinkResult) {
        if other.is_accepted() {
            *self = other;
        }
    }
    
    pub fn accept(&mut self) {
        *self = SinkResult::Accepted;
    }
}
