//! Imports hot-reloadable code.

// Ensure hot-reload features are mutually exclusive and at least one is enabled.
#[cfg(all(not(feature = "hot"), not(feature = "not-hot")))]
compile_error!("one of features `hot` or `not-hot` must be enabled");
#[cfg(all(feature = "hot", feature = "not-hot"))]
compile_error!("features `hot` and `not-hot` are mutually exclusive");

/// Actual `hot-lib-reloader`-imported module.
#[cfg(feature = "hot")]
#[hot_lib_reloader::hot_module(
    dylib = "chipbox_hot",
    file_watch_debounce = 50,
    lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug")
)]
mod hot {
    hot_functions_from_file!("chipbox-hot/src/lib.rs");
}

/// Re-export contents of the hot-reloadable module.
#[cfg(feature = "hot")]
pub use hot::*;

/// Re-export static module for compatibility with local/hot module from `hot-reload` feature.
#[cfg(feature = "not-hot")]
#[allow(unused_imports)]
pub(crate) use chipbox_hot::*;
