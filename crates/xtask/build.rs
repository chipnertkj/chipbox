use chipbox_build::{
    build_script, lockfile,
    miette::{self, Context as _},
};

/// # Responsibilities:
/// - Verify `crossterm` and `ratatui/crossterm` versions match.
fn main() -> miette::Result<()> {
    build_script::rerun_on_lockfile_change();
    let lockfile = lockfile::load_workspace().wrap_err("failed to read lockfile")?;
    lockfile::assert_versions_match(&lockfile, "ratatui/crossterm", "crossterm")
        .wrap_err("failed to verify crossterm versions match")?;
    Ok(())
}
