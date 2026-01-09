//! Native modules for the JS runtime.
//!
//! This module provides built-in modules exposed to JavaScript via the
//! `chipbox:*` namespace. It also provides factory functions for creating
//! the resolver and loader needed to register these modules with `rquickjs`.

use rquickjs::loader::{BuiltinResolver, ModuleLoader};

pub mod tracing;

macro_rules! modules {
    ($($name:ident),* $(,)?) => {
        /// Create a resolver for all native `chipbox:*` modules.
        pub fn resolver() -> BuiltinResolver {
            let mut resolver = BuiltinResolver::default();
            $(
                resolver.add_module(concat!("/@id/chipbox:", stringify!($name)));
            )*
            resolver
        }

        /// Create a loader for all native `chipbox:*` modules.
        pub fn loader() -> ModuleLoader {
            let mut loader = ModuleLoader::default();
            $(
                loader.add_module(
                    concat!("/@id/chipbox:", stringify!($name)),
                    self::$name::JsModule {}
                );
            )*
            loader
        }
    };
}

modules!(tracing);
