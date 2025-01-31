use std::{sync::Arc, time::Duration};

use clap::Parser as _;
use miette::{Context as _, IntoDiagnostic as _};
use watchexec::Watchexec;
use watchexec_events::{Event, Priority};

mod cargo_util;
mod cli;
mod quit_signal;
mod signature;
mod tracing_util;
mod watch;

fn force_working_dir() -> miette::Result<()> {
    cargo_util::project_to_working_dir()?;
    let (is_chipbox_root, _) = watch::is_chipbox_root()?;
    if !is_chipbox_root {
        Err(miette::miette!(
            help = format!("make sure you're in the root directory of the project"),
            "not in repo root"
        ))
    } else {
        tracing::debug!("working dir is repo root");
        Ok(())
    }
}

fn new_wx_rt(crates: watch::CrateNames, rt_type: watch::Handler) -> miette::Result<Arc<Watchexec>> {
    let job_id = Default::default();
    let wx = {
        let rt_type_clone = rt_type.clone();
        Watchexec::new_async(move |mut action| {
            let rt_type_clone = rt_type_clone.clone();
            Box::new(async move {
                quit_signal::handle(&mut action);
                rt_type_clone.handle_changes(&mut action, job_id).await;
                action
            })
        })?
    };
    wx.config.throttle(Duration::from_millis(50));
    wx.config.on_error(|hook| {
        tracing::error!("watchexec error: {}", hook.error);
    });
    let pathset: Vec<_> = crates
        .iter()
        .flat_map(|name| {
            tracing::debug!("rt `{rt_type:?}` is watching crate `{name}`");
            watch::crate_name_to_pathset(name)
        })
        .collect();
    wx.config.pathset(pathset);
    Ok(wx)
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let args = cli::Args::parse();
    tracing_util::init(args.verbose || cfg!(debug_assertions))?;
    force_working_dir()?;
    tracing::debug!("working dir: {:?}", std::env::current_dir());

    let wx_bin = new_wx_rt(watch::bin_crates(), watch::Handler::Bin)?;
    let wx_hot = new_wx_rt(watch::hot_crates(), watch::Handler::Hot {
        bin_watcher: Some(Arc::clone(&wx_bin)),
    })?;

    let send_empty_msg = |wx: Arc<Watchexec>| async move {
        wx.send_event(Event::default(), Priority::Urgent)
            .await
            .unwrap();
    };

    tracing::info!("starting dev tool");
    let rt = tokio::runtime::Handle::current();
    let (bin_result, hot_result, _, _) = tokio::join!(
        rt.spawn(wx_bin.main()),
        rt.spawn(wx_hot.main()),
        send_empty_msg(wx_bin),
        send_empty_msg(wx_hot)
    );
    bin_result
        .into_diagnostic()
        .wrap_err("failed to join bin rt task")?
        .into_diagnostic()
        .wrap_err("failed to join bin rt main")?
        .wrap_err("bin rt failed")?;
    tracing::debug!("bin rt exited");
    hot_result
        .into_diagnostic()
        .wrap_err("failed to join hot rt task")?
        .into_diagnostic()
        .wrap_err("failed to join hot rt main")?
        .wrap_err("hot rt failed")?;
    tracing::debug!("hot rt exited");

    // tracing::info!("starting ui");
    Ok(())
}
