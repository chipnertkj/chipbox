use crate::cargo_util::crate_folder_exists;
use crate::signature::DynlibChecker;
use miette::{Context as _, IntoDiagnostic as _};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::task::JoinSet;
use watchexec::{
    Watchexec,
    action::ActionHandler,
    command::{Command, SpawnOptions},
};
use watchexec_events::{Event, Priority};

pub(crate) type CrateNames = &'static [&'static str];

pub(crate) const fn bin_crates() -> CrateNames {
    &[
        "chipbox",
        "chipbox-synth",
        "chipbox-song",
        "chipnertkj-ui",
        "chipnertkj-ui-render",
    ]
}

pub(crate) const fn hot_crates() -> CrateNames {
    &["chipbox-hot", "chipnertkj-ui"]
}

#[derive(Clone, Debug)]
pub(crate) enum Handler {
    Bin,
    Hot { bin_watcher: Option<Arc<Watchexec>> },
}

impl Handler {
    async fn hot(bin_wx: Arc<Watchexec>, action: &mut ActionHandler, job_id: watchexec::Id) {
        let (prog, args) = ("cargo", vec!["build", "-p", "chipbox-hot"]);
        let job = action.get_or_create_job(job_id, || {
            let command = Command {
                program: watchexec::command::Program::Exec {
                    prog: prog.into(),
                    args: args.iter().map(|s| s.to_string()).collect(),
                },
                options: SpawnOptions {
                    grouped: true,
                    session: false,
                    reset_sigmask: false,
                },
            };
            command.into()
        });

        job.restart();
        job.to_wait().await;

        if action.paths().next().is_some()
            || action.events.iter().any(|event| event.tags.is_empty())
        {
            tracing::info!("rt `hot` rebuild");

            // For hot reloads, check if symbols were removed and trigger bin rebuild if needed
            {
                let lib_paths = [
                    std::path::PathBuf::from("target/debug/chipbox_hot.dll"),
                    std::path::PathBuf::from("target/debug/chipbox_hot-hot-3.dll"),
                ];
                let checkers = lib_paths
                    .into_iter()
                    .map(|path| DynlibChecker::new(path.clone()))
                    .collect::<Vec<_>>();
                // Spawn an async task to check symbols and trigger rebuild if needed
                let mut set = checkers
                    .into_iter()
                    .map(|checker| {
                        let bin_wx = bin_wx.clone();
                        async move {
                            match checker.check_for_removals().await {
                                Ok(true) => {
                                    tracing::info!(
                                        "hot reload removed symbols - triggering full rebuild"
                                    );
                                    // Send an empty event to trigger bin rebuild
                                    if let Err(e) =
                                        bin_wx.send_event(Event::default(), Priority::Urgent).await
                                    {
                                        tracing::error!("failed to send rebuild event: {}", e);
                                    }
                                }
                                Ok(false) => {
                                    tracing::debug!("no symbol removals detected");
                                }
                                Err(e) => {
                                    tracing::error!("failed to check for symbol removals: {}", e);
                                }
                            }
                        }
                    })
                    .collect::<JoinSet<_>>();
                while let Some(result) = set.join_next().await {
                    result.expect("failed to join hot reload checker task");
                }
            }
        }
    }

    pub(crate) async fn bin(action: &mut ActionHandler, job_id: watchexec::Id) {
        let (prog, args) = ("cargo", vec![
            "run",
            "-p",
            "chipbox",
            "-F",
            "hot",
            "--no-default-features",
        ]);
        let job = action.get_or_create_job(job_id, || {
            let command = Command {
                program: watchexec::command::Program::Exec {
                    prog: prog.into(),
                    args: args.iter().map(|s| s.to_string()).collect(),
                },
                options: SpawnOptions {
                    grouped: true,
                    session: false,
                    reset_sigmask: false,
                },
            };
            command.into()
        });

        job.restart();
    }

    pub(crate) async fn handle_changes(&self, action: &mut ActionHandler, job_id: watchexec::Id) {
        match self {
            Handler::Bin => {
                Self::bin(action, job_id).await;
            }
            Handler::Hot {
                bin_watcher: Some(bin_wx),
            } => {
                Self::hot(bin_wx.clone(), action, job_id).await;
            }
            Handler::Hot { bin_watcher: None } => {
                todo!()
            }
        }
    }
}

fn has_crates(parent: impl AsRef<Path>, crates: CrateNames) -> bool {
    crates
        .iter()
        .any(|name| crate_folder_exists(parent.as_ref(), name))
}

pub(crate) fn is_chipbox_root() -> miette::Result<(bool, PathBuf)> {
    let working_dir = std::env::current_dir()
        .into_diagnostic()
        .wrap_err("failed to get current dir")?;
    let has_bin_crates = has_crates(&working_dir, bin_crates());
    let has_hot_crates = has_crates(&working_dir, hot_crates());
    let condition = has_bin_crates && has_hot_crates;
    Ok((condition, working_dir))
}

pub(crate) fn crate_name_to_pathset(name: &str) -> Vec<PathBuf> {
    let manifest = Path::new(name).join("Cargo.toml");
    let src = Path::new(name).join("src");
    vec![manifest, src]
}
