use miette::{Context as _, IntoDiagnostic as _};

fn main() -> miette::Result<()> {
    chipbox::init::hooks(
        chipbox::init::TracingFilterBuilder::new()
            .with_self_directive(tracing::Level::DEBUG)
            .with_directive("js", tracing::Level::DEBUG)
            .build_from_default_env(),
    )
    .wrap_err("init main hooks")?;

    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .into_diagnostic()
        .wrap_err("build tokio runtime")?;

    let js_thread = start_js_thread(tokio_rt.handle().clone());
    js_thread
        .join()
        .expect("js thread panicked")
        .wrap_err("run js task")?;

    Ok(())
}

fn start_js_thread(
    tokio_rt: tokio::runtime::Handle,
) -> std::thread::JoinHandle<miette::Result<()>> {
    std::thread::spawn(move || {
        let js_set = tokio::task::LocalSet::new();
        let join_handle = js_set.run_until(js_task(tokio_rt.clone()));
        tokio_rt.block_on(join_handle)
    })
}

/// ## `!Send`
/// This task uses a `QuickJS` runtime, which is `!Send`.
#[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
async fn js_task(tokio_rt: tokio::runtime::Handle) -> miette::Result<()> {
    let js_rt = chipbox::js::runtime::JsRuntime::new(tokio_rt.clone())
        .await
        .into_diagnostic()
        .wrap_err("create js runtime")?;
    let result = js_rt.async_eval("await import('/src/main.tsx');").await;
    if let Err(chipbox::js::runtime::JsRuntimeError::EvalException(ref e)) = result {
        e.print_stack_trace();
    }
    let () = result.into_diagnostic().wrap_err("eval entry js")?;
    chipbox::js::repl(&js_rt).await
}
