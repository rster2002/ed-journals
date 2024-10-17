use serde::Serialize;

/// Returned from a resolver, which indicates whether an input has been accepted, skipped or should
/// be processed at a later time.
#[derive(Debug, Serialize)]
pub enum FeedResult {
    /// The input has been accepted by the resolver.
    Accepted,

    /// The input has been skipped by the resolver and should not be queued for later processing.
    Skipped,

    /// The input has been skipped by the resolver but should be queued so that it can be processed
    /// later.
    Later,
}

impl FeedResult {
    /// Whether the input has been accepted by the resolver.
    pub fn is_accepted(&self) -> bool {
        matches!(self, FeedResult::Accepted)
    }

    /// Whether the input should be processed later.
    pub fn is_later(&self) -> bool {
        matches!(self, FeedResult::Later)
    }

    /// Whether the input was skipped by the resolver.
    pub fn is_skipped(&self) -> bool {
        matches!(self, FeedResult::Skipped)
    }
}
