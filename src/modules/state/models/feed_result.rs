use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum FeedResult {
    Accepted,
    Skipped,
    Later,
}

impl FeedResult {
    pub fn is_accepted(&self) -> bool {
        matches!(self, FeedResult::Accepted)
    }

    pub fn is_later(&self) -> bool {
        matches!(self, FeedResult::Later)
    }

    pub fn is_skipped(&self) -> bool {
        matches!(self, FeedResult::Skipped)
    }
}
