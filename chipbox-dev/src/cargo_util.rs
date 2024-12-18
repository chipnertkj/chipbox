use miette::{miette, Context as _, IntoDiagnostic as _};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn project_root() -> miette::Result<PathBuf> {
    let output = Command::new("cargo")
        .arg("locate-project")
        .arg("--message-format")
        .arg("plain")
        .output()
        .into_diagnostic()
        .wrap_err("failed to run `cargo locate-project`")?;

    let string = String::from_utf8(output.stdout)
        .into_diagnostic()
        .wrap_err("failed to parse `cargo locate-project` output")?;

    let manifest_path = PathBuf::from(string.trim());
    let path = manifest_path
        .parent()
        .ok_or_else(|| miette!("project root does not exist"))?
        .to_owned();
    Ok(path)
}

pub(crate) fn project_to_working_dir() -> miette::Result<()> {
    let root = project_root().wrap_err("failed to find project root")?;
    tracing::debug!("setting working dir to {}", root.display());
    std::env::set_current_dir(root)
        .into_diagnostic()
        .wrap_err("failed to set working dir")?;
    Ok(())
}

pub(crate) fn crate_folder_exists(parent: &Path, name: &str) -> bool {
    parent.join(name).join("Cargo.toml").exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_root() {
        let root = project_root().unwrap();
        println!("cargo project root: {}", root.display());
        assert!(root.join("Cargo.toml").exists());
    }
}
