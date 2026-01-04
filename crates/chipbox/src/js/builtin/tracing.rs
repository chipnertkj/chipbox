use rquickjs::{
    Ctx, Error, Function,
    module::{Declarations, Exports, ModuleDef},
};

pub type JsModule = TracingModule;

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
