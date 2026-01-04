//! Native modules for the JS runtime.
//!
//! This module provides built-in modules exposed to JavaScript via the
//! `chipbox:*` namespace. It also provides factory functions for creating
//! the resolver and loader needed to register these modules with rquickjs.
//!
//! # Available Modules
//!
//! - `chipbox:tracing` - Logging through the chipbox tracing infrastructure

mod tracing;

use rquickjs::loader::{BuiltinResolver, ModuleLoader};

/// Create a resolver for all native `chipbox:*` modules.
pub fn resolver() -> BuiltinResolver {
    BuiltinResolver::default().with_module(tracing::MODULE_NAME)
}

/// Create a loader for all native `chipbox:*` modules.
pub fn loader() -> ModuleLoader {
    ModuleLoader::default().with_module(tracing::MODULE_NAME, tracing::TracingModule)
}
