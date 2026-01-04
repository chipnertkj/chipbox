mod builtin;
mod vite;

pub use vite::{ViteLoaderError, ViteLoaderResult};

pub fn loaders(
    tokio_rt: tokio::runtime::Handle,
) -> ViteLoaderResult<impl rquickjs::loader::Loader> {
    Ok((builtin::loader(), vite::loader(tokio_rt)?))
}

pub fn resolvers() -> impl rquickjs::loader::Resolver {
    (builtin::resolver(), vite::resolver())
}
