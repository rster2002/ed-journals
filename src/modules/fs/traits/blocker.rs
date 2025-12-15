use crate::fs::traits::unblocker::Unblocker;

/// A generic blocker independent of sync or async code providing a way to unblock it.
pub trait Blocker {
    fn unblocker(&self) -> Box<dyn Unblocker>;
}