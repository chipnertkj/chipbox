use std::time::Duration;
use tokio::sync::oneshot::{self, error::TryRecvError};

pub struct ObserverHandle {
    tx: oneshot::Sender<()>,
}

impl ObserverHandle {
    pub fn init(rt: &tokio::runtime::Runtime, on_reload: impl Fn() + Send + 'static) -> Self {
        let (tx, rx) = oneshot::channel::<()>();
        let _handle = rt.spawn_blocking(move || Self::observer(rx, on_reload));
        tracing::debug!("hot reload observer thread spawned");
        Self { tx }
    }

    fn observer(mut rx: oneshot::Receiver<()>, on_reload: impl Fn() + Send + 'static) {
        let reload_observer = crate::hot::subscribe();
        loop {
            if let Some(_guard) =
                reload_observer.wait_for_about_to_reload_timeout(Duration::from_millis(50))
            {
                tracing::debug!("hot reload detected");
                on_reload();
            }
            match rx.try_recv() {
                Err(TryRecvError::Empty) => {}
                _ => break,
            }
        }
        tracing::debug!("hot reload observer thread aborted");
    }

    pub fn abort(self) {
        tracing::debug!("aborting hot reload observer thread");
        self.tx
            .send(())
            .expect("failed to send abort to reload observer");
    }
}
