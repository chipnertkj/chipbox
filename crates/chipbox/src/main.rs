use miette::{Context as _, IntoDiagnostic as _};

fn main() -> miette::Result<()> {
    chipbox::init::hooks(
        chipbox::init::TracingFilterBuilder::new()
            .with_self_directive(tracing::Level::TRACE)
            .with_directive("js", tracing::Level::TRACE)
            .build_from_default_env(),
    )
    .wrap_err("init main hooks")?;

    let tokio_rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .into_diagnostic()
        .wrap_err("build tokio runtime")?;

    #[cfg(debug_assertions)]
    {
        let (hmr_client, hmr_recv) = tokio_rt
            .block_on(chipbox_js::HmrClient::new(24678))
            .wrap_err("create hmr client")?;

        let hmr_task = tokio_rt.spawn(hmr_client.start());
        let tokio_handle = tokio_rt.handle().clone();
        let js_thread = start_js_thread(tokio_handle, hmr_recv);

        let js_join = tokio_rt.spawn_blocking(move || js_thread.join());
        let (hmr_result, js_join_result) = tokio_rt
            .block_on(async move { tokio::try_join!(hmr_task, js_join) })
            .into_diagnostic()
            .wrap_err("try join hmr and js thread")?;

        let js_result = js_join_result.expect("js thread panicked");
        js_result.wrap_err("run js thread")?;
        hmr_result.wrap_err("run hmr task")?;
    }

    #[cfg(not(debug_assertions))]
    {
        let js_thread = start_js_thread(tokio_rt.handle().clone());
        js_thread
            .join()
            .expect("js thread panicked")
            .wrap_err("run js task")?;
    }

    Ok(())
}

fn start_js_thread(
    tokio_rt: tokio::runtime::Handle,
    #[cfg(debug_assertions)] hmr_recv: chipbox_js::HmrRecv,
) -> std::thread::JoinHandle<miette::Result<()>> {
    std::thread::spawn(move || {
        let js_set = tokio::task::LocalSet::new();
        let join_handle = js_set.run_until(js_task(
            #[cfg(debug_assertions)]
            hmr_recv,
        ));
        tokio_rt.block_on(join_handle)
    })
}

/// ## `!Send`
/// This task uses a `QuickJS` runtime, which is `!Send`.
#[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
async fn js_task(#[cfg(debug_assertions)] mut hmr_recv: chipbox_js::HmrRecv) -> miette::Result<()> {
    let mut js_rt = chipbox_js::Runtime::new()
        .await
        .into_diagnostic()
        .wrap_err("create js runtime")?;

    // Load entry point from Vite dev server
    #[cfg(debug_assertions)]
    load_entry_module(&js_rt).await?;

    // Run HMR event loop
    #[cfg(debug_assertions)]
    {
        while let Some(event) = hmr_recv.recv().await {
            let result = js_rt.handle_hmr_event(event).await;
            match result {
                Ok(true) => {}
                Ok(false) => {
                    tracing::info!("full reload requested, resetting context");
                    js_rt
                        .reset_runtime()
                        .await
                        .into_diagnostic()
                        .wrap_err("reset context")?;
                    load_entry_module(&js_rt).await?;
                }
                Err(e) => {
                    tracing::error!("hmr event error");
                    let stack_trace = e.stack_trace().map(ToString::to_string);
                    eprintln!("{:?}", miette::Report::from_err(e));
                    if let Some(stack_trace) = stack_trace {
                        eprintln!("stack trace: {stack_trace}");
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(debug_assertions)]
#[allow(clippy::future_not_send, reason = "rquickjs runtime is !Send")]
async fn load_entry_module(js_rt: &chipbox_js::Runtime) -> miette::Result<()> {
    const ENTRY_MODULE: &str = "/src/main.tsx";
    let result = js_rt.load_vite_module(ENTRY_MODULE).await;
    if let Err(ref e) = result
        && let Some(stack_trace) = e.stack_trace()
    {
        eprintln!("stack trace: {stack_trace}");
    }
    result.into_diagnostic().wrap_err("load entry module")?;
    Ok(())
}
