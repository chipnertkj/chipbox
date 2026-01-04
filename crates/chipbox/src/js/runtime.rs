use rquickjs::CatchResultExt as _;

use crate::js::exception::JsException;

#[derive(Debug, thiserror::Error)]
pub enum JsRuntimeError {
    #[error("init rquickjs runtime")]
    InitRquickjsRuntime(#[source] rquickjs::Error),
    #[error("init rquickjs context")]
    InitRquickjsContext(#[source] rquickjs::Error),
    #[error("exception caught from rquickjs eval")]
    EvalException(#[source] JsException),
    #[error("rquickjs eval error")]
    EvalError(#[source] rquickjs::Error),
    #[error("convert from js value")]
    FromJsValue(#[source] rquickjs::Error),
    #[error("vite loader error")]
    ViteLoaderError(#[from] crate::js::modules::ViteLoaderError),
}

impl From<rquickjs::CaughtError<'_>> for JsRuntimeError {
    fn from(e: rquickjs::CaughtError<'_>) -> Self {
        match e {
            rquickjs::CaughtError::Exception(ref e) => {
                Self::EvalException(JsException::from_js_exception(e))
            }
            rquickjs::CaughtError::Value(ref v) => {
                Self::EvalException(JsException::from_js_value(v))
            }
            rquickjs::CaughtError::Error(e) => Self::EvalError(e),
        }
    }
}

pub type JsRuntimeResult<T> = Result<T, JsRuntimeError>;

pub struct JsRuntime {
    rquickjs_rt: rquickjs::AsyncRuntime,
    rquickjs_ctx: rquickjs::AsyncContext,
    tokio_rt: tokio::runtime::Handle,
}

impl JsRuntime {
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn new(tokio_rt: tokio::runtime::Handle) -> JsRuntimeResult<Self> {
        let rquickjs_rt = Self::new_runtime(tokio_rt.clone()).await?;
        let rquickjs_ctx = Self::new_context(&rquickjs_rt).await?;
        let js_rt = Self {
            rquickjs_rt,
            rquickjs_ctx,
            tokio_rt,
        };
        Ok(js_rt)
    }

    /// Create a new `QuickJS` runtime.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    async fn new_runtime(
        tokio_rt: tokio::runtime::Handle,
    ) -> JsRuntimeResult<rquickjs::AsyncRuntime> {
        let rquickjs_rt =
            rquickjs::AsyncRuntime::new().map_err(JsRuntimeError::InitRquickjsRuntime)?;
        let resolvers = crate::js::modules::resolvers();
        let loaders = crate::js::modules::loaders(tokio_rt)?;
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
    ) -> JsRuntimeResult<rquickjs::AsyncContext> {
        rquickjs::AsyncContext::full(rquickjs_rt)
            .await
            .map_err(JsRuntimeError::InitRquickjsContext)
    }

    /// Reset the `QuickJS` context.
    /// If you want to reset the runtime as well, use [`Self::reset_runtime`] instead.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn reset_context(&mut self) -> JsRuntimeResult<()> {
        self.rquickjs_ctx = Self::new_context(&self.rquickjs_rt).await?;
        Ok(())
    }

    /// Reset the `QuickJS` runtime.
    ///
    /// ## `!Send`
    /// This future uses a `QuickJS` runtime, which is `!Send`.
    #[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
    pub async fn reset_runtime(&mut self) -> JsRuntimeResult<()> {
        self.rquickjs_rt = Self::new_runtime(self.tokio_rt.clone()).await?;
        self.reset_context().await?;
        Ok(())
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
    ) -> JsRuntimeResult<V> {
        let value = self
            .with_eval(source, |ctx, js_value| {
                V::from_js(&ctx, js_value).map_err(JsRuntimeError::FromJsValue)
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
    ) -> JsRuntimeResult<V> {
        let value = self
            .with_async_eval(source, |ctx, js_value| {
                V::from_js(&ctx, js_value).map_err(JsRuntimeError::FromJsValue)
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
    pub async fn with_eval<R, F>(&self, source: impl Into<Vec<u8>>, f: F) -> JsRuntimeResult<R>
    where
        R: 'static,
        F: for<'js> FnOnce(rquickjs::Ctx<'js>, rquickjs::Value<'js>) -> JsRuntimeResult<R>,
    {
        rquickjs::async_with!(self.rquickjs_ctx => |ctx| {
            let js_value = ctx.eval(source).catch(&ctx).map_err(JsRuntimeError::from)?;
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
    pub async fn with_async_eval<R, F>(
        &self,
        source: impl Into<Vec<u8>>,
        f: F,
    ) -> JsRuntimeResult<R>
    where
        R: 'static,
        F: for<'js> FnOnce(rquickjs::Ctx<'js>, rquickjs::Value<'js>) -> JsRuntimeResult<R>,
    {
        rquickjs::async_with!(self.rquickjs_ctx => |ctx| {
            let promise = ctx.eval_promise(source).catch(&ctx).map_err(JsRuntimeError::from)?;
            let mut js_value: rquickjs::Value = promise.into_future().await.catch(&ctx).map_err(JsRuntimeError::from)?;
            if let Some(obj) = js_value.as_object()
                && let Ok(inner) = obj.get("value") {
                    js_value = inner;
                }
            f(ctx, js_value)
        })
        .await
    }
}

/// A utility trait required due to [`rquickjs::FromJs`] binding the [`rquickjs::Ctx`]'s lifetime to the value.
pub trait FromJsExt: Sized {
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
