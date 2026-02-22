use crate::fs::traits::unblocker::Unblocker;

/// A generic blocker independent of sync or async code providing a way to unblock it. You can use
/// this trait to implement your own blocking logic if you want to use a custom implementation.
pub trait Blocker {
    /// Return an unblocker which can be called to unblock this blocker.
    fn unblocker(&self) -> Box<dyn Unblocker>;
}
