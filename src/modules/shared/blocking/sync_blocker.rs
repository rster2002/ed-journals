use std::sync::{Arc, Mutex, RwLock, Weak};
use std::thread;
use std::thread::Thread;

#[derive(Debug, Clone)]
pub struct SyncBlocker {
    waiting_thread: Arc<Mutex<(Option<Thread>,)>>,
    children: Arc<RwLock<Vec<Weak<SyncBlocker>>>>,
}

impl SyncBlocker {
    pub fn new() -> Self {
        SyncBlocker {
            waiting_thread: Arc::new(Mutex::new((None,))),
            children: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn unblock(&self) {
        let mut guard = self.waiting_thread.lock().expect("Should have been locked");

        if let Some(thread) = guard.0.as_ref() {
            thread.unpark();
            guard.0 = None;

            self.unblock_children();
        };
    }

    pub fn unblock_children(&self) {
        let iter = self.children.try_read()
            .expect("Should have been acquired");

        dbg!(&iter);

        for child in iter.iter() {
            dbg!("Handling child");
            if let Some(child) = child.upgrade() {
                dbg!(&child);
                child.unblock();
            }
        }
    }

    pub fn block(&self) {
        {
            let mut guard = self.waiting_thread.lock().expect("to have gotten a lock");

            guard.0 = Some(thread::current());
        }

        thread::park();
    }

    pub fn child(&mut self) -> Arc<SyncBlocker> {
        let child = Arc::new(SyncBlocker::new());

        self.children.write()
            .expect("Failed to acquire lock")
            .push(Arc::downgrade(&child));

        dbg!(child)
    }
}
