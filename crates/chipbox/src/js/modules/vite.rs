mod loader {
    //! Vite module loader for rquickjs.
    //!
    //! Uses [`Handle::spawn`] to fetch modules on the main runtime, then
    //! [`blocking_recv`] to wait for results. This is safe because the loader
    //! runs on a dedicated OS thread, not a tokio async worker.
    //!
    //! [`Handle::spawn`]: tokio::runtime::Handle::spawn
    //! [`blocking_recv`]: tokio::sync::oneshot::Receiver::blocking_recv

    #[derive(Debug, thiserror::Error)]
    #[error("vite loader error")]
    pub enum ViteLoaderError {
        #[error("init http client")]
        Init(#[source] reqwest::Error),
        #[error("parse url")]
        ParseUrl(#[from] url::ParseError),
        #[error("request error")]
        RequestError(#[source] reqwest::Error),
        #[error("response text error")]
        ResponseTextError(#[source] reqwest::Error),
        #[error("response status error")]
        ResponseStatusError(reqwest::StatusCode),
        #[error("channel closed")]
        ChannelClosed(#[source] oneshot::RecvError),
    }

    pub type ViteLoaderResult<T> = Result<T, ViteLoaderError>;

    use std::time::Duration;

    use rquickjs::{Ctx, Module, loader::Loader};

    /// Fetches transformed ES modules from Vite dev server.
    pub struct ViteLoader {
        tokio_rt: tokio::runtime::Handle,
        client: reqwest::Client,
        base_url: reqwest::Url,
    }

    impl ViteLoader {
        pub fn new(tokio_rt: tokio::runtime::Handle) -> ViteLoaderResult<Self> {
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .map_err(ViteLoaderError::Init)?;
            let loader = Self {
                tokio_rt,
                base_url: Self::base_url(),
                client,
            };
            tracing::info!("initialized vite loader");
            Ok(loader)
        }

        /// Vite dev server address.
        #[must_use]
        fn base_url() -> reqwest::Url {
            reqwest::Url::parse("http://localhost:5173").expect("valid url")
        }

        fn build_url(&self, path: &str) -> ViteLoaderResult<reqwest::Url> {
            let s = format!("{}{}", self.base_url.as_str().trim_end_matches('/'), path);
            let url = reqwest::Url::parse(&s)?;
            Ok(url)
        }

        async fn fetch(client: &reqwest::Client, url: reqwest::Url) -> ViteLoaderResult<String> {
            let result = client
                .get(url.clone())
                .header("Accept", "application/javascript")
                .send()
                .await;
            let response = result.map_err(ViteLoaderError::RequestError)?;
            let status = response.status();
            if status.is_success() {
                let result = response.text().await;
                let source = result.map_err(ViteLoaderError::ResponseTextError)?;
                Ok(source)
            } else {
                Err(ViteLoaderError::ResponseStatusError(status))
            }
        }
    }

    impl Loader for ViteLoader {
        fn load<'js>(
            &mut self,
            ctx: &Ctx<'js>,
            name: &str,
        ) -> Result<Module<'js>, rquickjs::Error> {
            tracing::debug!(name, "fetching module");
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
                eprintln!("{:?}", miette::Report::from_err(e));
                rquickjs::Error::new_loading(name)
            })?;
            // Block waiting - std::sync::mpsc doesn't care about tokio context
            tracing::trace!(name, bytes = source.len(), "declaring module");
            Module::declare(ctx.clone(), name, source)
        }
    }
}
mod resolve {
    //! Vite module resolver for rquickjs.
    //!
    //! Resolves ES module specifiers to paths that can be fetched from the Vite dev server.

    use rquickjs::{Ctx, Error, loader::Resolver};

    /// Resolves module specifiers to Vite-compatible paths.
    ///
    /// Vite transforms modules and serves them with special handling:
    /// - Bare specifiers like `"solid-js"` -> pre-bundled in `node_modules/.vite/deps/`
    /// - Relative imports `"./foo"` -> resolved relative to importer
    /// - Absolute paths `"/src/main.tsx"` -> used as-is
    #[derive(Default)]
    pub struct ViteResolver;

    impl ViteResolver {
        #[must_use]
        pub const fn new() -> Self {
            Self
        }
    }

    impl Resolver for ViteResolver {
        fn resolve(&mut self, _ctx: &Ctx<'_>, base: &str, name: &str) -> Result<String, Error> {
            tracing::trace!("resolve {name} in {base}");
            // Native chipbox:* modules are handled by `ModuleLoader`, not `ViteLoader`.
            if name.starts_with("/@id/chipbox:") {
                tracing::debug!("resolving chipbox:* module {name}");
                return Ok(name
                    .strip_prefix("/@id/")
                    .expect("checked prefix")
                    .to_string());
            }

            let resolved = if name.starts_with("./") || name.starts_with("../") {
                // Relative import - resolve against base's directory
                resolve_relative(base, name)
            } else if name.starts_with('/') {
                // Absolute path - use as-is
                name.to_string()
            } else {
                // Bare specifier (e.g., "solid-js", "chipbox-solid-render")
                // Vite pre-bundles these into node_modules/.vite/deps/
                // The exact path depends on how Vite transforms them
                // For now, we'll let Vite handle the resolution by prefixing with /@id/
                // which tells Vite to resolve it as a bare import
                format!("/@id/{name}")
            };

            tracing::trace!("resolved {name} to {resolved} in {base}");
            Ok(resolved)
        }
    }

    /// Resolve a relative path against a base path.
    fn resolve_relative(base: &str, relative: &str) -> String {
        // Get the directory of the base path
        let base_dir = base.rfind('/').map_or("", |idx| &base[..idx]);

        // Split relative path into components
        let mut components: Vec<&str> = if base_dir.is_empty() {
            Vec::new()
        } else {
            base_dir.split('/').filter(|s| !s.is_empty()).collect()
        };

        for part in relative.split('/') {
            match part {
                "" | "." => {}
                ".." => {
                    components.pop();
                }
                other => components.push(other),
            }
        }

        if components.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", components.join("/"))
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_resolve_relative() {
            assert_eq!(resolve_relative("/src/main.tsx", "./App"), "/src/App");
            assert_eq!(
                resolve_relative("/src/components/Button.tsx", "../utils/helpers"),
                "/src/utils/helpers"
            );
            assert_eq!(
                resolve_relative("/src/main.tsx", "./lib/foo"),
                "/src/lib/foo"
            );
        }
    }
}

pub use loader::{ViteLoader, ViteLoaderError, ViteLoaderResult};
pub use resolve::ViteResolver;

pub fn loader(tokio_rt: tokio::runtime::Handle) -> ViteLoaderResult<ViteLoader> {
    ViteLoader::new(tokio_rt)
}

pub const fn resolver() -> ViteResolver {
    ViteResolver::new()
}
