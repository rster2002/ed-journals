use std::fmt::{Debug, Formatter};
use std::mem;
use std::ops::{Deref, DerefMut};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::state::models::feed_result::FeedResult;
use crate::state::models::resolvers::game_state_resolver::GameStateResolver;
use crate::state::traits::state_resolver::StateResolver;

/// This is what is used internally take care of input and manage flushing and handling retries.
pub struct StateContainer<S, T>
    where S : StateResolver<T>,
          T : Clone,
{
    /// The inner resolver which is what actually takes input and does something with it. The state
    /// container calls the methods on the given [StateResolver].
    inner: S,

    /// Some events might get queued for later processing because not all information might be
    /// available at the time.
    later: Vec<T>,
}

impl<S, T> StateContainer<S, T>
    where S : StateResolver<T>,
          T : Clone,
{
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
    /// [StateContainer::feed] as that will likely cause performance issues.
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

impl<S, T> DerefMut for StateContainer<S, T>
where S : StateResolver<T>,
      T : Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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

impl<S, T> Serialize for StateContainer<S, T>
where S : StateResolver<T> + Serialize,
      T : Clone
{
    fn serialize<Se>(&self, serializer: Se) -> Result<Se::Ok, Se::Error>
    where
        Se: Serializer
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, S, T> Deserialize<'de> for StateContainer<S, T>
where S : StateResolver<T> + Deserialize<'de>,
      T : Clone
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Ok(StateContainer {
            inner: S::deserialize(deserializer)?,
            later: Vec::new(),
        })
    }
}

impl<S, T> From<S> for StateContainer<S, T>
    where S : StateResolver<T>,
          T : Clone,
{
    fn from(value: S) -> Self {
        StateContainer {
            inner: value,
            later: Vec::new(),
        }
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
