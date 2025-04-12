use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Waker};
use tokio::sync::mpsc::{channel, Sender};

#[derive(Debug, Default, Clone)]
pub struct AsyncBlocker {
    waker: Arc<Mutex<Option<Waker>>>,
}

pub struct AsyncBlock {
    waker: Arc<Mutex<Option<Waker>>>,
}

impl AsyncBlocker {
    #[deprecated]
    pub fn new() -> AsyncBlocker {
        AsyncBlocker::default()
    }

    pub fn unblock(&self) {
        let mut guard = self.waker.lock()
            .expect("Waker mutex poisoned");

        if guard.is_none() {
            return;
        }

        let waker = mem::take(&mut *guard)
            .expect("should have been checked");

        waker.wake();
    }

    pub fn block(&self) -> AsyncBlock {
        AsyncBlock {
            waker: Arc::clone(&self.waker),
        }
    }
}

impl Future for AsyncBlock {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut mut_ref = self.waker
            .lock()
            .expect("poisoned async blocker");

        let _ = mem::replace(&mut *mut_ref, Some(cx.waker().clone()));

        Poll::Pending
    }
}

// impl AsyncBlocker {
//     pub fn new() -> Self {
//         AsyncBlocker {
//             waiting_sender: Arc::new(Mutex::new((None,))),
//         }
//     }
//
//     pub async fn block(&self) {
//         let (sender, mut receiver) = channel(2);
//
//         {
//             let mut guard = self.waiting_sender.lock()
//                 .await;
//
//             guard.0 = Some(sender);
//         }
//
//         receiver
//             .recv()
//             .await
//             .expect("Failed to perform async block");
//     }
//
//     pub async fn unblock(&self) {
//         let mut guard = self.waiting_sender
//             .lock()
//             .await;
//
//         if let Some(sender) = &guard.0 {
//             if sender.is_closed() {
//                 return;
//             }
//
//             sender.send(()).await.expect("Failed to send");
//
//             guard.0 = None;
//         }
//     }
//
//     pub fn unblock_blocking(&self) {
//         futures::executor::block_on(async {
//             self.unblock().await;
//         });
//         dbg!("Finished blocking");
//     }
// }
