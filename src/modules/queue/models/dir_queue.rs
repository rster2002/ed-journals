use std::collections::VecDeque;
use crate::modules::queue::DirEntry;

pub struct DirQueue {
    inner: VecDeque<DirEntry>
}

impl DirQueue {
    pub fn new() -> Self {
        DirQueue {}
    }
}