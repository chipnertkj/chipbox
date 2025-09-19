use chipbox_build::{
    lockfile,
    miette::{self, Context as _},
};

/// # Responsibilities:
/// - Verify `wgpu` and `vello/wgpu` versions match.
fn main() -> miette::Result<()> {
    lockfile::set_rerun_on_change();
    let lockfile = lockfile::read().wrap_err("failed to read lockfile")?;
    lockfile::assert_versions_match(&lockfile, "wgpu", "vello/wgpu")
        .wrap_err("failed to verify wgpu versions match")?;
    Ok(())
}
