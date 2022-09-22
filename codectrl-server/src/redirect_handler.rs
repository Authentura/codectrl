use futures::Future;
use log::{debug, info, trace};
use parking_lot::Mutex;
use serde_json::json;
use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};
use tokio::{
    runtime::Handle,
    sync::{
        oneshot::{self, Sender as OneshotSender},
        watch::{self, Receiver as WatchReceiver},
    },
    task::JoinHandle,
    time::{sleep, Duration},
};
use warp::Filter;

struct ThreadHandle<T: Send + Sync + 'static> {
    is_closed_rx: Mutex<Option<WatchReceiver<bool>>>,
    inner: Mutex<Option<JoinHandle<T>>>,
}

impl<T: Send + Sync + 'static> ThreadHandle<T> {
    pub fn none() -> Self {
        Self {
            inner: Mutex::new(None),
            is_closed_rx: Mutex::new(None),
        }
    }

    pub fn init<F>(&self, handle: &Handle, is_closed_rx: WatchReceiver<bool>, f: F)
    where
        F: Future<Output = T> + Send + 'static,
    {
        *self.is_closed_rx.lock() = Some(is_closed_rx);
        let mut lock = self.inner.lock();

        if lock.is_none() {
            info!(target: "codectrl_server - redirect handler", "Handle not spawned. Spawning now...");
            *lock = Some(handle.spawn(f));
        } else {
            debug!(target: "codectrl_server - redirect handler", "Handle already spawned");
        }
    }

    pub fn is_started(&self) -> bool {
        let lock = self.is_closed_rx.lock();
        trace!(target: "codectrl_server - redirect handler", "Got is_closed_rx lock");

        if let Some(rx) = &*lock {
            let val = *rx.borrow();
            trace!(target: "codectrl_server - redirect handler", "is_closed_rx: {val}");
            !val
        } else {
            false
        }
    }

    pub fn close(&self, tx: OneshotSender<()>) {
        let mut lock = self.inner.lock();

        if let Some(_handle) = &mut *lock {
            let res = tx.send(());

            trace!(target: "codectrl_server - redirect handler", "Result of signal send: {res:#?}");
            debug!(target: "codectrl_server - redirect handler", "Sent shutdown signal...");
        }
    }
}

pub struct RedirectHandler {
    port: u16,
    inner_handle: ThreadHandle<()>,
    count: AtomicU16,
}

impl RedirectHandler {
    #[must_use]
    pub fn new(port: u16) -> Arc<Self> {
        Arc::new(Self {
            port,
            inner_handle: ThreadHandle::none(),
            count: AtomicU16::new(0),
        })
    }

    pub fn start(self: &Arc<Self>, runtime_handle: Handle) {
        let arc_self = Arc::clone(self);

        let (is_closed_sender, is_closed_receiver) = watch::channel(false);

        self.inner_handle.init(&runtime_handle.clone(), is_closed_receiver, async move {
            let arc_self_clone = Arc::clone(&arc_self);
            let (close_signal_sender, close_signal_receiver) = oneshot::channel();

            runtime_handle.spawn(async move {
                // wait 5 seconds before auto-closing the inner_handle. gives time for a slow or
                // manual request
               sleep(Duration::new(5, 0)).await; 

                loop {
                    if arc_self_clone.count.load(Ordering::SeqCst) == 0 {
                        info!(target: "codectrl_server - redirect handler", "Closing temporary Warp server...");
                        arc_self_clone.close(close_signal_sender);
                        info!(target: "codectrl_server - redirect handler", "Test");
                        break;
                    }
                }
            });

            let register = warp::path!("register" / String).map(|provider| {
                warp::reply::json(&json!({ "message": format!("Hello {provider}") }))
            });

            info!(target: "codectrl_server - redirect handler", "Spinning up temporary Warp server on 127.0.0.1:{}", arc_self.port);

            let (_, server) = warp::serve(register)
                .bind_with_graceful_shutdown(([127, 0, 0, 1], arc_self.port), async move {
                    _ = is_closed_sender.send(false);
                    close_signal_receiver.await.ok();
                    info!(target: "codectrl_server - redirect handler", "Closed temporary Warp server");
                    _ = is_closed_sender.send(true);
                });

            server.await;
        });

        loop {
            if self.is_started() {
                break;
            }
        }
    }

    pub fn register(&self) { _ = self.count.fetch_add(1, Ordering::SeqCst); }

    pub fn unregister(&self) {
        if self.count.load(Ordering::SeqCst) > 0 {
            self.count.fetch_sub(1, Ordering::SeqCst);
        }
    }

    pub fn is_started(&self) -> bool { self.inner_handle.is_started() }

    fn close(&self, tx: OneshotSender<()>) { self.inner_handle.close(tx); }
}
