use miette::{Context as _, IntoDiagnostic as _};

fn main() -> miette::Result<()> {
    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .into_diagnostic()
        .wrap_err("build tokio runtime")?;
    chipbox::init::hooks(
        chipbox::init::TracingFilterBuilder::new()
            .with_self_directive(tracing::Level::DEBUG)
            .with_directive("js", tracing::Level::DEBUG)
            .build_from_default_env(),
    )
    .wrap_err("init main hooks")?;
    tokio_rt.block_on(async {
        let js_runtime = chipbox_js::Runtime::new()
            .await
            .into_diagnostic()
            .wrap_err("create js runtime")?;
        chipbox_js::repl(&js_runtime).await
    })
}
