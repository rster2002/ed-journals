use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use crate::state::models::feed_result::FeedResult;
use crate::state::models::resolvers::game_state_resolver::GameStateResolver;
use crate::state::traits::state::StateResolver;

pub struct StateContainer<S, T>
    where S : StateResolver<T>,
          T : Clone,
{
    inner: S,
    later: Vec<T>,
}

impl<S, T> StateContainer<S, T>
    where S : StateResolver<T>,
          T : Clone,
{
    pub fn new(inner: S) -> Self {
        StateContainer {
            inner,
            later: Vec::new(),
        }
    }

    /// Takes the log events and processes it in the state. Note that it does not guarantee that the
    /// event will be processed immediately. In some situations the event will be queued when the
    /// state things it is better able to process the event, but it doesn't do this automatically.
    /// For those events to be processed, you need to call [StateContainer::flush]. This will go through
    /// the remaining events and tries to process them.
    pub fn feed(&mut self, input: &T) {
        let handle_result = self.inner.feed(&input);

        if let FeedResult::Later = handle_result {
            self.later.push(input.clone());
        }
    }

    /// Processes any left-over events that were scheduled for later processing. Call this sparingly
    /// especially not while you're also still reading a lot of events through
    /// [GameStateResolver::feed_log_event] as that will likely cause performance issues.
    pub fn flush(&mut self) {
        let queued = mem::take(&mut self.later);

        for item in queued {
            if let FeedResult::Later = self.inner.feed(&item) {
                self.later.push(item);
            }
        }

        self.inner.flush_inner();
    }
}

impl<S, T> Deref for StateContainer<S, T>
    where S : StateResolver<T>,
          T : Clone,
{
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S, T> Default for StateContainer<S, T>
    where S : StateResolver<T> + Default,
          T : Clone
{
    fn default() -> Self {
        StateContainer {
            inner: S::default(),
            later: Vec::new(),
        }
    }
}

impl<S, T, I> From<I> for StateContainer<S, T>
where S : StateResolver<T> + From<I>,
      T : Clone
{
    fn from(value: I) -> Self {
        StateContainer {
            inner: S::from(value),
            later: Vec::new(),
        }
    }
}

impl<S, T, I, E> TryFrom<I> for StateContainer<S, T>
where S : StateResolver<T> + TryFrom<I, Error = E>,
      T : Clone
{
    type Error = E;

    fn try_from(value: I) -> Result<Self, Self::Error> {
        Ok(StateContainer {
            inner: S::try_from(value)?,
            later: Vec::new(),
        })
    }
}

impl<S, T> Debug for StateContainer<S, T>
where S : StateResolver<T> + Debug,
      T : Clone
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::LogState;

    #[test]
    fn test() {
        let i = LogState::default();
    }
}
