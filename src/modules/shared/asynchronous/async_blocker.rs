use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::{channel, Sender};

#[derive(Debug, Clone)]
pub struct AsyncBlocker {
    waiting_sender: Arc<Mutex<(Option<Sender<()>>,)>>,
}

impl AsyncBlocker {
    pub fn new() -> Self {
        AsyncBlocker {
            waiting_sender: Arc::new(Mutex::new((None,))),
        }
    }

    pub async fn block(&self) {
        let (sender, mut receiver) = channel(2);

        {
            let mut guard = self.waiting_sender.lock().expect("to gotten lock");

            guard.0 = Some(sender);
        }

        receiver
            .recv()
            .await
            .expect("Failed to perform async block");
    }

    pub fn unblock_blocking(&self) {
        let mut guard = self.waiting_sender.lock().expect("Should have been locked");

        if let Some(sender) = guard.0.as_ref() {
            if sender.is_closed() {
                return;
            }

            sender.blocking_send(()).expect("Failed to send");

            guard.0 = None;
        }
    }
}
