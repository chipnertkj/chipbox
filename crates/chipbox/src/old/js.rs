//! Self-contained JS runtime for executing the `SolidJS` frontend.
//!
//! The JS worker runs a dedicated OS thread with a [`LocalSet`] for `!Send`
//! [`rquickjs`] futures. Communication done via channels.
//!
//! # Architecture
//!
//! ```text
//! Main tokio runtime
//! ├── HMR task (async, runs on main)
//! └── JS thread (std::thread)
//!     └── LocalSet (block_on via Handle)
//!         └── rquickjs AsyncRuntime (!Send)
//! ```
//!
//! [`LocalSet`]: tokio::task::LocalSet

use crate::TokioHandle;

pub mod hmr;
pub mod loader;
pub mod native;

mod executor;

/// Configuration for a JS worker.
pub struct JsWorkerConfig {
    /// Entry module to load.
    pub entry_module: &'static str,
}

impl Default for JsWorkerConfig {
    fn default() -> Self {
        Self {
            entry_module: "/src/main.tsx",
        }
    }
}

/// Self-contained execution environment for JS code.
pub struct JsWorker {
    /// Join handle for the OS thread.
    thread_handle: std::thread::JoinHandle<miette::Result<()>>,
}

impl JsWorker {
    /// Spawn a [`JsWorker`].
    pub fn new(tokio_rt: TokioHandle, hmr_rx: hmr::EventRx) -> Self {
        let executor = executor::JsExecutor::new(tokio_rt, JsWorkerConfig::default(), hmr_rx);
        let thread_handle = std::thread::spawn(move || executor.run());
        Self { thread_handle }
    }

    pub fn join(self) -> miette::Result<()> {
        self.thread_handle
            .join()
            .map_err(|e| miette::miette!(help = "JS thread panicked", "{e:?}"))?
    }
}
