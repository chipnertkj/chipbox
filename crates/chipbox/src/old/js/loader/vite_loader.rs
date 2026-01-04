//! Vite module loader for rquickjs.
//!
//! Uses [`Handle::spawn`] to fetch modules on the main runtime, then
//! [`blocking_recv`] to wait for results. This is safe because the loader
//! runs on a dedicated OS thread, not a tokio async worker.
//!
//! [`Handle::spawn`]: tokio::runtime::Handle::spawn
//! [`blocking_recv`]: tokio::sync::oneshot::Receiver::blocking_recv

use std::time::Duration;

use miette::{Context as _, IntoDiagnostic as _};
use rquickjs::{Ctx, Module, loader::Loader};

use crate::TokioHandle;

/// Fetches transformed ES modules from Vite dev server.
pub struct ViteLoader {
    tokio_rt: TokioHandle,
    client: reqwest::Client,
    base_url: reqwest::Url,
}

impl ViteLoader {
    pub fn new(tokio_rt: TokioHandle) -> miette::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .into_diagnostic()
            .wrap_err("init http client")?;
        Ok(Self {
            tokio_rt,
            base_url: Self::base_url(),
            client,
        })
    }

    /// Vite dev server address.
    #[must_use]
    fn base_url() -> reqwest::Url {
        reqwest::Url::parse("http://localhost:5173").expect("valid url")
    }

    fn build_url(&self, path: &str) -> miette::Result<reqwest::Url> {
        let s = format!("{}{}", self.base_url.as_str().trim_end_matches('/'), path);
        reqwest::Url::parse(&s)
            .into_diagnostic()
            .wrap_err("parse url")
    }

    async fn fetch(client: &reqwest::Client, url: reqwest::Url) -> miette::Result<String> {
        let result = client
            .get(url.clone())
            .header("Accept", "application/javascript")
            .send()
            .await;
        let response = result.into_diagnostic().wrap_err("request error")?;
        let status = response.status();
        if status.is_success() {
            let result = response.text().await;
            result.into_diagnostic().wrap_err("await response text")
        } else {
            Err(miette::miette!(url = url, "failed to fetch: {status}"))
        }
    }
}

impl Loader for ViteLoader {
    fn load<'js>(&mut self, ctx: &Ctx<'js>, name: &str) -> Result<Module<'js>, rquickjs::Error> {
        tracing::trace!(name, "fetching module");
        let url = self
            .build_url(name)
            .map_err(|e| rquickjs::Error::new_loading(format!("{e}")))?;
        let (tx, rx) = oneshot::channel();
        self.tokio_rt.spawn({
            let client = self.client.clone();
            async move {
                let source = Self::fetch(&client, url).await;
                tx.send(source).expect("channel closed");
            }
        });
        let source = rx.recv().expect("channel closed").map_err(|e| {
            tracing::error!(name, "fetch failed");
            eprintln!("{e:?}");
            rquickjs::Error::new_loading(name)
        })?;
        // Block waiting - std::sync::mpsc doesn't care about tokio context
        tracing::trace!(name, bytes = source.len(), "declaring module");
        Module::declare(ctx.clone(), name, source)
    }
}
