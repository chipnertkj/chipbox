use chipbox_build::{build_script, lockfile, miette};

/// # Responsibilities:
/// - Verify all immediate dependency versions are consistent with transitive deps.
fn main() -> miette::Result<()> {
    build_script::rerun_on_lockfile_change();
    lockfile::assert_deps_consistency(&lockfile::load_workspace()?)?;
    Ok(())
}
