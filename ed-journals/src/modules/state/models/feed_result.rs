#[derive(Debug)]
pub enum FeedResult {
    Accepted,
    Skipped,
    Later,
}

impl FeedResult {
    pub fn is_accepted(&self) -> bool {
        matches!(self, FeedResult::Accepted)
    }
}
