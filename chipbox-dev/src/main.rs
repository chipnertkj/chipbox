use std::{sync::Arc, time::Duration};

use clap::Parser as _;
use miette::{Context as _, IntoDiagnostic as _};
use watchexec::Watchexec;
use watchexec_events::{Event, Priority};

mod cargo_util;
mod cli;
mod quit;
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

fn new_wx_rt(
    crates: watch::CrateNames,
    rt_type: watch::WxRtType,
) -> miette::Result<Arc<Watchexec>> {
    let job_id = Default::default();
    let wx = Watchexec::new(move |mut action| {
        quit::handle(&mut action);
        rt_type.handle_changes(&mut action, job_id);
        action
    })?;
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
    tracing_util::init(args.verbose)?;
    force_working_dir()?;

    let wx_bin = new_wx_rt(watch::bin_crates(), watch::WxRtType::Bin)?;
    let wx_hot = new_wx_rt(watch::hot_crates(), watch::WxRtType::Hot)?;

    let send_msg = |wx: Arc<Watchexec>| async move {
        wx.send_event(Event::default(), Priority::Urgent)
            .await
            .unwrap();
    };

    tracing::info!("starting dev tool");
    let (bin_result, hot_result, _, _) = tokio::join!(
        wx_bin.main(),
        wx_hot.main(),
        send_msg(wx_bin),
        send_msg(wx_hot)
    );
    bin_result
        .into_diagnostic()
        .wrap_err("failed to join bin rt")?
        .into_diagnostic()
        .wrap_err("bin rt failed")?;
    tracing::debug!("bin rt exited");
    hot_result
        .into_diagnostic()
        .wrap_err("failed to join hot rt")?
        .into_diagnostic()
        .wrap_err("hot rt failed")?;
    tracing::debug!("hot rt exited");

    // tracing::info!("starting ui");
    Ok(())
}
