use crate::cargo_util::crate_folder_exists;
use miette::{Context as _, IntoDiagnostic as _};
use std::path::{Path, PathBuf};
use watchexec::{
    action::ActionHandler,
    command::{Command, SpawnOptions},
};

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

#[derive(Copy, Clone, Debug)]
pub(crate) enum WxRtType {
    Bin,
    Hot,
}

impl WxRtType {
    pub(crate) fn handle_changes(&self, action: &mut ActionHandler, job_id: watchexec::Id) {
        let (prog, args) = match self {
            WxRtType::Bin => ("cargo", vec![
                "run",
                "-p",
                "chipbox",
                "-F",
                "hot",
                "--no-default-features",
            ]),
            WxRtType::Hot => ("cargo", vec!["build", "-p", "chipbox-hot"]),
        };
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

        if action.paths().next().is_some()
            || action.events.iter().any(|event| event.tags.is_empty())
        {
            tracing::info!("rt `{self:?}` rebuild");
            job.restart();
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
