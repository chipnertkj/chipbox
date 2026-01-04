//! Native tracing module for JS runtime.
//!
//! Exposes Rust tracing macros to JavaScript, allowing TS code to log through
//! the chipbox tracing infrastructure.
//!
//! # Usage
//!
//! ```typescript
//! import { info, debug, warn, error, trace } from "chipbox:tracing";
//!
//! info("Hello from TypeScript!");
//! debug("Debug message");
//! ```

use rquickjs::{
    Ctx, Error, Function,
    module::{Declarations, Exports, ModuleDef},
};

/// Module name for the tracing module.
pub const MODULE_NAME: &str = "/@id/chipbox:tracing";

/// Native module exposing tracing functions to JS.
pub struct TracingModule;

impl ModuleDef for TracingModule {
    fn declare(decl: &Declarations<'_>) -> Result<(), Error> {
        decl.declare("trace")?;
        decl.declare("debug")?;
        decl.declare("info")?;
        decl.declare("warn")?;
        decl.declare("error")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<(), Error> {
        exports.export(
            "trace",
            Function::new(ctx.clone(), |msg: String| {
                tracing::trace!(target: "js", "{msg}");
            })?,
        )?;

        exports.export(
            "debug",
            Function::new(ctx.clone(), |msg: String| {
                tracing::debug!(target: "js", "{msg}");
            })?,
        )?;

        exports.export(
            "info",
            Function::new(ctx.clone(), |msg: String| {
                tracing::info!(target: "js", "{msg}");
            })?,
        )?;

        exports.export(
            "warn",
            Function::new(ctx.clone(), |msg: String| {
                tracing::warn!(target: "js", "{msg}");
            })?,
        )?;

        exports.export(
            "error",
            Function::new(ctx.clone(), |msg: String| {
                tracing::error!(target: "js", "{msg}");
            })?,
        )?;

        Ok(())
    }
}
