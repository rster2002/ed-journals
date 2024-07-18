use crate::state::models::feed_result::FeedResult;
use crate::state::models::state_container::StateContainer;

/// Implemented for state which is wrapped in a [StateContainer]. Defines how the state should
/// handle and flush underlying input. These methods should usually not be called directly. Instead,
/// use [StateContainer::feed] and [StateContainer::flush].
pub trait StateResolver<T> {
    /// Feeds an input to the resolver and process it. If the function returns [FeedResult::Later]
    /// it will be queued to be processed later using the [StateContainer::flush] method.
    fn feed(&mut self, input: &T) -> FeedResult;

    /// If the model has any other underlying states this method can be used to call
    /// [StateContainer::flush] on those states. NOTE to not call this method directly: always use
    /// [StateContainer::flush] instead.
    fn flush_inner(&mut self) {}
}
