use std::sync::{Arc};
use futures::lock::Mutex;
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
            let mut guard = self.waiting_sender.lock()
                .await;

            guard.0 = Some(sender);
        }

        receiver
            .recv()
            .await
            .expect("Failed to perform async block");
    }

    pub async fn unblock(&self) {
        let mut guard = self.waiting_sender
            .lock()
            .await;

        if let Some(sender) = &guard.0 {
            if sender.is_closed() {
                return;
            }

            sender.send(()).await.expect("Failed to send");

            guard.0 = None;
        }
    }

    pub fn unblock_blocking(&self) {
        futures::executor::block_on(async {
            self.unblock().await;
        });
        dbg!("Finished blocking");
    }
}
