use crate::state::models::feed_result::FeedResult;

pub trait StateResolver<T> {
    fn feed(&mut self, input: &T) -> FeedResult;

    fn flush_inner(&mut self) {}
}
