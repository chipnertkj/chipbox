use chipbox_build::{build_script, lockfile, miette};

fn main() -> miette::Result<()> {
    build_script::rerun_on_script_change();
    lockfile::assert_deps_consistency(&lockfile::load_workspace()?)?;
    Ok(())
}
