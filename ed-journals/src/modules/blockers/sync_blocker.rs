use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::Thread;

#[derive(Debug, Clone)]
pub struct SyncBlocker {
    waiting_thread: Arc<Mutex<(Option<Thread>,)>>,
}

impl SyncBlocker {
    pub fn new() -> Self {
        SyncBlocker {
            waiting_thread: Arc::new(Mutex::new((None,))),
        }
    }

    pub fn unblock(&self) {
        {
            let mut guard = self.waiting_thread.lock().expect("to have gotten a lock");

            guard.0 = Some(thread::current());
        }

        thread::park();
    }

    pub fn block(&self) {
        let mut guard = self.waiting_thread
            .lock()
            .expect("Should have been locked");

        if let Some(thread) = guard.0.as_ref() {
            thread.unpark();
            guard.0 = None;
        };
    }
}
