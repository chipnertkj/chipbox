//! Vite dev server integration for module loading.

use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};

use rquickjs::{Ctx, Function, Object, Promise, Value};

use crate::runtime::{QjsError, QjsResult, QjsResultExt};

/// Vite dev server base URL.
const VITE_BASE_URL: &str = "http://localhost:5173";

/// HTTP client for fetching modules from Vite dev server.
static CLIENT: LazyLock<Arc<reqwest::Client>> = LazyLock::new(|| Arc::new(init_client()));

#[derive(Debug, thiserror::Error)]
pub enum RegistryInitError {
    #[error("init module registry global")]
    Registry(#[source] QjsError),
    #[error("init require global")]
    Require(#[source] QjsError),
}

impl RegistryInitError {
    pub fn stack_trace(&self) -> Option<&str> {
        match self {
            Self::Registry(e) | Self::Require(e) => e.stack_trace(),
        }
    }
}

pub type RegistryInitResult<T> = Result<T, RegistryInitError>;

fn init_client() -> reqwest::Client {
    reqwest::Client::builder()
        .connect_timeout(Duration::from_secs(1))
        .timeout(Duration::from_secs(2))
        .build()
        .expect("init http client")
}

fn init_registry<'js>(ctx: &Ctx<'js>, globals: &Object<'js>) -> QjsResult<()> {
    let registry = Object::new(ctx.clone()).catch(ctx)?;
    globals.set("__qjs_module_registry", registry).catch(ctx)?;
    Ok(())
}

fn init_require(ctx: &Ctx<'_>) -> QjsResult<()> {
    let func = Function::new(ctx.clone(), require);
    ctx.globals().set("__qjs_require", func).catch(ctx)?;
    Ok(())
}

#[allow(
    clippy::needless_pass_by_value,
    reason = "FromJsFunc requires FromJs types"
)]
fn require(ctx: Ctx<'_>, path: String) -> rquickjs::Result<Promise<'_>> {
    let globals = ctx.globals();
    let registry: Object = globals.get("__qjs_module_registry")?;
    let module: Value = registry.get(&path)?;
    let fut = {
        let ctx = ctx.clone();
        async move {
            if module.is_undefined() {
                let code = fetch_module(&path).await?;
                tracing::trace!(path, "eval");
                let promise: Promise = ctx.eval(code)?;
                let module: Object = promise.into_future().await?;
                registry.set(&path, module)?;
                let module: Object = registry.get(&path)?;
                let exports = module.get("exports")?;
                Ok::<Object, rquickjs::Error>(exports)
            } else {
                let module = module.try_into_object().map_err(|_| {
                    rquickjs::Error::new_loading(format!("module was not an object: {path}"))
                })?;
                let exports = module.get("exports")?;
                Ok(exports)
            }
        }
    };
    let promise = Promise::wrap_future(&ctx, fut)?;
    Ok(promise)
}

/// Initialize Vite module globals in the given context.
pub fn init_globals(ctx: &Ctx<'_>) -> RegistryInitResult<()> {
    let globals = ctx.globals();
    init_registry(ctx, &globals).map_err(RegistryInitError::Registry)?;
    init_require(ctx).map_err(RegistryInitError::Require)?;
    Ok(())
}

/// Invalidate a module from the registry cache.
///
/// Returns `true` if the module was in the cache and removed, `false` if it wasn't cached.
pub fn invalidate_module(ctx: &Ctx<'_>, path: &str) -> QjsResult<bool> {
    let globals = ctx.globals();
    let registry: Object = globals.get("__qjs_module_registry").catch(ctx)?;
    let module: Value = registry.get(path).catch(ctx)?;
    if module.is_undefined() {
        Ok(false)
    } else {
        registry.remove(path).catch(ctx)?;
        tracing::debug!(path, "invalidated module");
        Ok(true)
    }
}

/// Async fetch module code from Vite dev server.
async fn fetch_module(path: &str) -> rquickjs::Result<String> {
    tracing::debug!(path, "fetch");
    // Request source.
    let url = format!("{VITE_BASE_URL}{path}");
    let client = CLIENT.clone();
    let request = client
        .get(&url)
        .header("Accept", "application/javascript")
        .timeout(Duration::from_secs(5));
    // Send request.
    let response = request
        .send()
        .await
        .map_err(|e| rquickjs::Error::Io(std::io::Error::other(e.to_string())))?
        .error_for_status()
        .map_err(|e| rquickjs::Error::Io(std::io::Error::other(e.to_string())))?;
    // Read body.
    response.text().await.map_err(|e| {
        let e = format!("failed to read body for {path}: {e}");
        rquickjs::Error::new_loading(e)
    })
}
