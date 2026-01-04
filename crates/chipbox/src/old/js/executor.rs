//! JS executor implementation using [`LocalSet`].
//!
//! # [`LocalSet`]
//!
//! The [`rquickjs`] [`AsyncRuntime`] and [`AsyncContext`] are `!Send` - they cannot
//! be shared across threads. We run them in a [`LocalSet`] on a dedicated OS
//! thread, which provides a single-threaded async executor for `!Send` futures.
//!
//! [`LocalSet`]: tokio::task::LocalSet
//! [`AsyncRuntime`]: rquickjs::AsyncRuntime
//! [`AsyncContext`]: rquickjs::AsyncContext
//! [`Loader`]: rquickjs::loader::Loader
//! [`blocking_recv`]: tokio::sync::oneshot::Receiver::blocking_recv

// NEXT: refactor js runtime so that its a nice api (restart, eval module, eval str, etc etc etc)

// TODO: move to better place
#[derive(Debug, thiserror::Error)]
#[error("{}", self.display_message())]
pub struct JsException {
    message: Option<String>,
    stack: Option<String>,
}

// TODO: move to better place
impl JsException {
    pub fn catch(ctx: &rquickjs::Ctx<'_>) -> Option<Self> {
        let value = ctx.catch();
        if value.is_null() {
            None
        } else if let Some(exception) = value.as_exception() {
            Some(Self {
                message: exception.message(),
                stack: exception.stack(),
            })
        } else {
            // https://www.reddit.com/r/copypasta/comments/1ei47t4/hate/
            panic!("non-exception value caught from rquickjs")
        }
    }

    fn display_message(&self) -> &str {
        self.message
            .as_deref()
            .unwrap_or("(exception without message)")
    }
}

// TODO: move to better place
#[derive(Debug, thiserror::Error)]
pub enum JsError {
    #[error("caught JS exception")]
    Exception(#[from] JsException),
    #[error("rquickjs error")]
    Other(#[from] rquickjs::Error),
}

pub type JsResult<T> = Result<T, JsError>;

#[derive(Debug, thiserror::Error)]
#[error("{}", self.message)]
pub struct ExecutorError {
    pub source: JsError,
    pub message: String,
}

impl ExecutorError {
    pub fn print(self: Arc<Self>) {
        eprintln!("{:?}", miette::Report::from_err(self.clone()));
        if let Self {
            source: JsError::Exception(e),
            ..
        } = self.as_ref()
            && let Some(ref stack) = e.stack
        {
            eprintln!("stack trace:\n{stack}");
        }
    }
}

pub type ExecutorResult<T> = Result<T, ExecutorError>;

pub trait JsResultExt<T> {
    fn executor_result(self, message: impl Into<String>) -> ExecutorResult<T>;
}

impl<T> JsResultExt<T> for JsResult<T> {
    fn executor_result(self, message: impl Into<String>) -> ExecutorResult<T> {
        self.map_err(|e| ExecutorError {
            source: e,
            message: message.into(),
        })
    }
}

// TODO: move to better place
pub trait RquickjsResultExt<T> {
    fn catch_js_exception(self, ctx: &rquickjs::Ctx<'_>) -> JsResult<T>;
}

impl<T> RquickjsResultExt<T> for rquickjs::Result<T> {
    #[allow(clippy::future_not_send, reason = "run in LocalSet")]
    fn catch_js_exception(self, ctx: &rquickjs::Ctx<'_>) -> JsResult<T> {
        self.map_err(|e| match e {
            rquickjs::Error::Exception => JsException::catch(ctx)
                .expect("must be some exception")
                .into(),
            other => other.into(),
        })
    }
}

use std::sync::Arc;

use miette::{Context as _, IntoDiagnostic as _};
use rquickjs::{AsyncContext, AsyncRuntime};
use tokio::task::LocalSet;

use super::{
    JsWorkerConfig,
    hmr::HmrEvent,
    loader::{ViteLoader, ViteResolver},
    native,
};
use crate::{TokioHandle, js::hmr};

/// Action to take after handling an HMR event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RuntimeAction {
    /// Continue with current runtime.
    Continue,
    /// Restart the entire JS runtime (fresh module cache).
    FullReload,
}

pub struct JsExecutor {
    tokio_rt: TokioHandle,
    worker_config: JsWorkerConfig,
    hmr_rx: hmr::EventRx,
}

impl JsExecutor {
    pub const fn new(
        tokio_rt: TokioHandle,
        worker_config: JsWorkerConfig,
        hmr_rx: hmr::EventRx,
    ) -> Self {
        Self {
            tokio_rt,
            worker_config,
            hmr_rx,
        }
    }

    /// Evaluate JavaScript code that returns a promise.
    ///
    /// The code is evaluated in a new module with the given name.
    /// Returns an error if evaluation fails or if the promise rejects.
    #[allow(clippy::future_not_send, reason = "run in LocalSet")]
    async fn eval_module(
        ctx: &AsyncContext,
        name: impl Into<String>,
        code: impl Into<String>,
        task_name: &str,
    ) -> ExecutorResult<()> {
        rquickjs::async_with!(ctx => |ctx| {
            rquickjs::Module::evaluate(ctx.clone(), name.into(), code.into())
                .map(rquickjs::Promise::into_future::<()>)
                .catch_js_exception(&ctx)
                .executor_result(format!("{task_name} - eval"))?
                .await
                .catch_js_exception(&ctx)
                .executor_result(format!("{task_name} - await"))
        })
        .await
    }

    pub fn run(self) -> miette::Result<()> {
        let local_set = LocalSet::new();
        let tokio_rt = self.tokio_rt.clone();
        tokio_rt.block_on(local_set.run_until(self.event_loop()))
    }

    /// JS executor event loop.
    ///
    /// Outer loop handles full reloads by recreating the runtime.
    /// Inner loop handles HMR updates within a single runtime instance.
    #[allow(clippy::future_not_send, reason = "run in LocalSet")]
    async fn event_loop(mut self) -> miette::Result<()> {
        'runtime: loop {
            let (js_rt, js_ctx) = self.init_runtime().await.wrap_err("init runtime")?;
            self.load_entry(&js_ctx)
                .await
                .into_diagnostic()
                .wrap_err("load entry module")?;

            tracing::debug!("entering event loop");
            loop {
                // `select!` polls both futures each iteration:
                // - Polling `drive()` makes progress on pending JS work (timers, promises)
                // - When an HMR event arrives, `select!` returns that branch immediately
                // - biased ensures HMR events are checked first (priority)
                tokio::select! {
                    biased;
                    message = self.hmr_rx.recv() => {
                        match message {
                            Some(event) => {
                                if self.handle_hmr_event(&js_ctx, event).await == RuntimeAction::FullReload {
                                    tracing::info!("full reload - restarting JS runtime");
                                    drop(js_ctx);
                                    drop(js_rt);
                                    continue 'runtime;
                                }
                            }
                            // HMR channel closed - exit completely
                            None => break 'runtime,
                        }
                    }
                    // `drive()` only completes when runtime is dropped
                    () = js_rt.drive() => break 'runtime,
                }
            }
        }

        Ok(())
    }

    /// Initialize rquickjs runtime with Vite loader and native modules.
    #[allow(clippy::future_not_send, reason = "run in LocalSet")]
    async fn init_runtime(&self) -> miette::Result<(AsyncRuntime, AsyncContext)> {
        // Initialize rquickjs runtime.
        let js_rt = AsyncRuntime::new()
            .into_diagnostic()
            .wrap_err("init rquickjs runtime")?;

        // Native module resolver/loader for chipbox:* modules.
        let native_resolver = native::resolver();
        let native_loader = native::loader();

        // Vite resolver/loader for external modules.
        let vite_resolver = ViteResolver::new();
        let vite_loader = ViteLoader::new(self.tokio_rt.clone()).wrap_err("init vite loader")?;

        // Combine resolvers and loaders (tuple implements Resolver/Loader traits).
        let resolvers = (native_resolver, vite_resolver);
        let loaders = (native_loader, vite_loader);

        js_rt.set_loader(resolvers, loaders).await;

        // Initialize rquickjs context.
        let js_ctx = AsyncContext::full(&js_rt)
            .await
            .into_diagnostic()
            .wrap_err("init rquickjs context")?;

        Ok((js_rt, js_ctx))
    }

    /// Load entry module via dynamic import.
    #[allow(clippy::future_not_send, reason = "run in LocalSet")]
    async fn load_entry(&self, js_ctx: &AsyncContext) -> ExecutorResult<()> {
        let entry_point = self.worker_config.entry_module;
        tracing::debug!(entry_point, "loading entry module");

        Self::eval_module(
            js_ctx,
            "__chipbox_bootstrap__",
            format!(r#"import("{entry_point}")"#),
            "import entry module",
        )
        .await?;

        tracing::debug!("entry module evaluated");
        Ok(())
    }

    /// Handle an HMR event. Returns the action to take.
    #[allow(clippy::future_not_send, reason = "run in LocalSet")]
    async fn handle_hmr_event(&self, ctx: &AsyncContext, event: HmrEvent) -> RuntimeAction {
        match event {
            HmrEvent::Update { paths } => {
                for path in paths {
                    let result = self.reload_module(ctx, &path).await;
                    if let Err(e) = result {
                        tracing::error!(path, "reload failed");
                        Arc::new(e).print();
                    }
                }
                RuntimeAction::Continue
            }
            HmrEvent::Prune { paths } => {
                for path in &paths {
                    if let Err(e) = self.prune_module(ctx, path).await {
                        tracing::error!(path, "prune failed");
                        Arc::new(e).print();
                    }
                }
                RuntimeAction::Continue
            }
            HmrEvent::FullReload => {
                tracing::warn!("full reload requested");
                RuntimeAction::FullReload
            }
        }
    }

    /// Trigger a module reload with cache-busting and HMR accept.
    #[allow(clippy::future_not_send, reason = "run in LocalSet")]
    async fn reload_module(&self, ctx: &AsyncContext, path: &str) -> ExecutorResult<()> {
        tracing::trace!(path, "reloading module");

        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        let import_path = if path.contains('?') {
            format!("{path}&t={ts}")
        } else {
            format!("{path}?t={ts}")
        };

        // 1. Trigger HMR accept callbacks FIRST (solid-refresh diffs old vs new)
        //    The old module's callbacks run before we import the new version.
        Self::eval_module(
            ctx,
            format!("__hmr_accept_{ts}__"),
            format!(r#"globalThis.__vite_hot_accept("{path}")"#),
            "trigger hmr accept",
        )
        .await?;

        // 2. Import the new module version (cache-busted)
        //    This registers fresh callbacks for the next HMR cycle.
        Self::eval_module(
            ctx,
            format!("__hmr_import_{ts}__"),
            format!(r#"import("{import_path}")"#),
            "import hmr-reloaded module",
        )
        .await?;

        tracing::debug!(path, "module reloaded");
        Ok(())
    }

    /// Prune a module - run dispose callbacks and remove from hot context registry.
    #[allow(clippy::future_not_send, reason = "run in LocalSet")]
    async fn prune_module(&self, ctx: &AsyncContext, path: &str) -> ExecutorResult<()> {
        tracing::trace!(path, "pruning module");

        // Call __vite_hot_prune to run dispose callbacks and remove from registry
        Self::eval_module(
            ctx,
            "__hmr_prune__",
            format!(r#"globalThis.__vite_hot_prune?.("{path}")"#),
            "trigger hmr prune",
        )
        .await?;

        tracing::debug!(path, "module pruned");
        Ok(())
    }
}
