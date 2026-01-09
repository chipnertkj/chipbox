#[cfg(debug_assertions)]
mod vite;

#[cfg(debug_assertions)]
use self::vite::RegistryInitError;
use crate::exception::JsException;
#[cfg(debug_assertions)]
use crate::hmr::{HmrEvent, HmrRecv};

#[derive(Debug, thiserror::Error)]
pub enum QjsError {
    /// Exception caught while performing an operation on the runtime.
    #[error("exception caught from qjs")]
    Exception(#[from] JsException),
    /// Error not caused by an exception.
    #[error("qjs runtime error")]
    Runtime(#[source] rquickjs::Error),
}

impl QjsError {
    #[must_use]
    pub fn stack_trace(&self) -> Option<&str> {
        match self {
            Self::Exception(e) => e.stack_trace(),
            Self::Runtime(_) => None,
        }
    }
}

impl From<rquickjs::CaughtError<'_>> for QjsError {
    fn from(e: rquickjs::CaughtError<'_>) -> Self {
        match e {
            rquickjs::CaughtError::Exception(ref e) => JsException::from_js_exception(e).into(),
            rquickjs::CaughtError::Value(ref v) => JsException::from_js_value(v).into(),
            rquickjs::CaughtError::Error(e) => Self::Runtime(e),
        }
    }
}

pub type QjsResult<T> = Result<T, QjsError>;

pub trait QjsResultExt<T> {
    fn with_no_ctx(self) -> Result<T, QjsError>;
    fn catch(self, ctx: &rquickjs::Ctx<'_>) -> Result<T, QjsError>;
}

impl<T> QjsResultExt<T> for rquickjs::Result<T> {
    fn with_no_ctx(self) -> Result<T, QjsError> {
        self.map_err(QjsError::Runtime)
    }

    fn catch(self, ctx: &rquickjs::Ctx<'_>) -> Result<T, QjsError> {
        rquickjs::CatchResultExt::catch(self, ctx).map_err(Into::into)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error("init qjs runtime")]
    InitQjsRuntime(#[source] QjsError),
    #[error("init qjs context")]
    InitQjsContext(#[source] QjsError),
    #[error("eval error")]
    Eval(#[source] QjsError),
    #[error("convert from js value")]
    FromJsValue(#[source] QjsError),
    // Vite module errors.
    #[cfg(debug_assertions)]
    #[error("init vite module registry")]
    InitRegistry(#[source] RegistryInitError),
    #[cfg(debug_assertions)]
    #[error("invalidate module")]
    InvalidateModule(#[source] QjsError),
    #[cfg(debug_assertions)]
    #[error("reload module")]
    ReloadModule(#[source] QjsError),
}

impl RuntimeError {
    #[must_use]
    pub fn stack_trace(&self) -> Option<&str> {
        match self {
            #[cfg(debug_assertions)]
            Self::InitRegistry(e) => e.stack_trace(),
            #[cfg(debug_assertions)]
            Self::InvalidateModule(e) | Self::ReloadModule(e) => e.stack_trace(),
            Self::Eval(e)
            | Self::FromJsValue(e)
            | Self::InitQjsRuntime(e)
            | Self::InitQjsContext(e) => e.stack_trace(),
        }
    }
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;

pub struct Runtime {
    rquickjs_rt: rquickjs::AsyncRuntime,
    rquickjs_ctx: rquickjs::AsyncContext,
    #[cfg(debug_assertions)]
    hmr_recv: Option<HmrRecv>,
}

impl Runtime {
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn new() -> RuntimeResult<Self> {
        let rquickjs_rt = Self::new_runtime().await?;
        let rquickjs_ctx = Self::new_context(&rquickjs_rt).await?;

        // Initialize vite globals (__qjs_require, __qjs_register_module)
        #[cfg(debug_assertions)]
        rquickjs::async_with!(rquickjs_ctx => |ctx| {
            vite::init_globals(&ctx)
        })
        .await
        .map_err(RuntimeError::InitRegistry)?;

        Ok(Self {
            rquickjs_rt,
            rquickjs_ctx,
            #[cfg(debug_assertions)]
            hmr_recv: None,
        })
    }

    /// ## Panics
    /// - If the HMR receiver is already set.
    #[cfg(debug_assertions)]
    #[must_use]
    pub fn with_hmr_recv(mut self, hmr_rx: HmrRecv) -> Self {
        assert!(self.hmr_recv.is_none(), "hmr recv already set");
        self.hmr_recv = Some(hmr_rx);
        self
    }

    /// Create a new `QuickJS` runtime.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    async fn new_runtime() -> RuntimeResult<rquickjs::AsyncRuntime> {
        let rquickjs_rt = rquickjs::AsyncRuntime::new()
            .with_no_ctx()
            .map_err(RuntimeError::InitQjsRuntime)?;
        let resolvers = crate::modules::resolver();
        let loaders = crate::modules::loader();
        rquickjs_rt.set_loader(resolvers, loaders).await;
        Ok(rquickjs_rt)
    }

    /// Create a new `QuickJS` context.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    async fn new_context(
        rquickjs_rt: &rquickjs::AsyncRuntime,
    ) -> RuntimeResult<rquickjs::AsyncContext> {
        let ctx = rquickjs::AsyncContext::full(rquickjs_rt)
            .await
            .with_no_ctx()
            .map_err(RuntimeError::InitQjsContext)?;
        // Re-initialize vite globals in the new context
        #[cfg(debug_assertions)]
        rquickjs::async_with!(ctx => |ctx| {
            vite::init_globals(&ctx)
        })
        .await
        .map_err(RuntimeError::InitRegistry)?;
        Ok(ctx)
    }

    /// Reset the `QuickJS` context.
    /// If you want to reset the runtime as well, use [`Self::reset_runtime`] instead.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn reset_context(&mut self) -> RuntimeResult<()> {
        self.rquickjs_ctx = Self::new_context(&self.rquickjs_rt).await?;
        Ok(())
    }

    /// Reset the `QuickJS` runtime.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn reset_runtime(&mut self) -> RuntimeResult<()> {
        self.rquickjs_rt = Self::new_runtime().await?;
        self.reset_context().await?;
        Ok(())
    }

    /// Load a module from Vite dev server.
    ///
    /// Calls `__qjs_require(path)` which handles fetching, evaluating, and
    /// caching the module and its dependencies.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[cfg(debug_assertions)]
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn load_vite_module(&self, path: &str) -> RuntimeResult<()> {
        let require_call = format!("await __qjs_require({path:?})");
        self.async_eval::<()>(require_call).await
    }

    /// Evaluate a string of JavaScript code in the current context.
    /// Then, convert the result to a Rust value and return it.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn eval<V: FromJsExt + 'static>(
        &self,
        source: impl Into<Vec<u8>>,
    ) -> RuntimeResult<V> {
        let value = self
            .with_eval(source, |ctx, js_value| {
                V::from_js(&ctx, js_value)
                    .catch(&ctx)
                    .map_err(RuntimeError::FromJsValue)
            })
            .await?;
        Ok(value)
    }

    /// Evaluate a string of JavaScript code in the current context, with top-level await support.
    /// Then, convert the result to a Rust value and return it.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn async_eval<V: FromJsExt + 'static>(
        &self,
        source: impl Into<Vec<u8>>,
    ) -> RuntimeResult<V> {
        let value = self
            .with_async_eval(source, |ctx, js_value| {
                V::from_js(&ctx, js_value)
                    .catch(&ctx)
                    .map_err(RuntimeError::FromJsValue)
            })
            .await?;
        Ok(value)
    }

    /// Evaluate a string of JavaScript code in the current context.
    /// Then, call the closure on the result of the script evaluation and return its result.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn with_eval<R, F>(&self, source: impl Into<Vec<u8>>, f: F) -> RuntimeResult<R>
    where
        R: 'static,
        F: for<'js> FnOnce(rquickjs::Ctx<'js>, rquickjs::Value<'js>) -> RuntimeResult<R>,
    {
        rquickjs::async_with!(self.rquickjs_ctx => |ctx| {
            let js_value = ctx.eval(source).catch(&ctx).map_err(RuntimeError::Eval)?;
            f(ctx, js_value)
        })
        .await
    }

    /// Evaluate a string of JavaScript code in the current context, with top-level await support.
    /// Then, call the closure on the result of the script evaluation and return its result.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn with_async_eval<R, F>(&self, source: impl Into<Vec<u8>>, f: F) -> RuntimeResult<R>
    where
        R: 'static,
        F: for<'js> FnOnce(rquickjs::Ctx<'js>, rquickjs::Value<'js>) -> RuntimeResult<R>,
    {
        rquickjs::async_with!(self.rquickjs_ctx => |ctx| {
            let promise = ctx.eval_promise(source).catch(&ctx).map_err(RuntimeError::Eval)?;
            let mut js_value: rquickjs::Value = promise.into_future().await.catch(&ctx).map_err(RuntimeError::Eval)?;
            if let Some(obj) = js_value.as_object()
                && let Ok(inner) = obj.get("value") {
                    js_value = inner;
                }
            f(ctx, js_value)
        })
        .await
    }

    /// Invalidate modules from the registry cache.
    ///
    /// Removes cached modules so they will be re-fetched on next import.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[cfg(debug_assertions)]
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn invalidate_modules(&self, paths: &[String]) -> RuntimeResult<()> {
        rquickjs::async_with!(self.rquickjs_ctx => |ctx| {
            for path in paths {
                vite::invalidate_module(&ctx, path)?;
            }
            Ok(())
        })
        .await
        .map_err(RuntimeError::InvalidateModule)?;
        Ok(())
    }

    /// Reload modules by invalidating, re-importing, and triggering HMR accept.
    ///
    /// This triggers Solid fast refresh for updated components.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[cfg(debug_assertions)]
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn reload_modules(&self, paths: &[String]) -> RuntimeResult<()> {
        for path in paths {
            // 1. Invalidate old module
            self.invalidate_modules(&[path.clone()]).await?;

            // 2. Re-import (this evaluates and caches new module)
            self.load_vite_module(path).await.map_err(|e| match e {
                RuntimeError::Eval(qjs_err) => RuntimeError::ReloadModule(qjs_err),
                other => other,
            })?;

            // 3. Trigger HMR accept with the new module
            self.eval_hmr_accept(path).await;
        }
        Ok(())
    }

    /// Prune modules by running dispose callbacks and removing from HMR registry.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[cfg(debug_assertions)]
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn prune_modules(&self, paths: &[String]) -> RuntimeResult<()> {
        self.invalidate_modules(paths).await?;
        for path in paths {
            self.eval_hmr_prune(path).await;
        }
        Ok(())
    }

    /// Trigger HMR accept callbacks for a module path.
    #[cfg(debug_assertions)]
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    async fn eval_hmr_accept(&self, path: &str) {
        let path = path.to_string();
        let result: QjsResult<bool> = {
            let path = path.clone();
            rquickjs::async_with!(self.rquickjs_ctx => |ctx| {
                // Get the module from registry
                let globals = ctx.globals();
                let registry: rquickjs::Object = globals.get("__qjs_module_registry").catch(&ctx)?;
                let module: rquickjs::Value = registry.get(&path).catch(&ctx)?;

                // Extract exports (or undefined if module not found)
                let exports: rquickjs::Value = if let Some(obj) = module.as_object() {
                    obj.get("exports").catch(&ctx)?
                } else {
                    rquickjs::Value::new_undefined(ctx.clone())
                };

                // Call __vite_hot_accept(path, exports)
                let accept_fn: rquickjs::Function = globals.get("__vite_hot_accept").catch(&ctx)?;
                let result: bool = accept_fn.call((path.as_str(), exports)).catch(&ctx)?;

                Ok(result)
            })
        }
        .await;

        match result {
            Ok(true) => tracing::debug!(path, "HMR accept"),
            Ok(false) => tracing::warn!(path, "HMR accept returned false"),
            Err(e) => {
                tracing::error!("hmr event error");
                let stack_trace = e.stack_trace().map(ToString::to_string);
                eprintln!("{:?}", miette::Report::from_err(e));
                if let Some(stack_trace) = stack_trace {
                    eprintln!("stack trace: {stack_trace}");
                }
            }
        }
    }

    /// Trigger HMR prune callbacks for a module path.
    #[cfg(debug_assertions)]
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    async fn eval_hmr_prune(&self, path: &str) {
        let call = format!("__vite_hot_prune({path:?})");
        let _ = self.async_eval::<()>(call).await;
    }

    /// Handle an HMR event.
    ///
    /// Returns `true` if the runtime should continue, `false` if a full reload is required.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[cfg(debug_assertions)]
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn handle_hmr_event(&self, event: HmrEvent) -> RuntimeResult<bool> {
        match event {
            HmrEvent::Update { paths } => {
                tracing::info!(?paths, "HMR update");
                self.reload_modules(&paths).await?;
                Ok(true)
            }
            HmrEvent::Prune { paths } => {
                tracing::info!(?paths, "HMR prune");
                self.prune_modules(&paths).await?;
                Ok(true)
            }
            HmrEvent::FullReload => {
                tracing::warn!("HMR full reload requested");
                Ok(false)
            }
        }
    }
}

/// A utility trait required due to [`rquickjs::FromJs`] binding the [`rquickjs::Ctx`]'s lifetime to the value.
pub trait FromJsExt: Sized {
    /// Convert a `QuickJS` value to a Rust value.
    ///
    /// ## Errors
    /// See [`rquickjs::FromJs`] for details.
    fn from_js<'js>(
        ctx: &rquickjs::Ctx<'js>,
        value: rquickjs::Value<'js>,
    ) -> rquickjs::Result<Self>;
}

impl<T> FromJsExt for T
where
    for<'js> T: rquickjs::FromJs<'js>,
{
    fn from_js<'js>(
        ctx: &rquickjs::Ctx<'js>,
        value: rquickjs::Value<'js>,
    ) -> rquickjs::Result<Self> {
        T::from_js(ctx, value)
    }
}
