use chipbox_build::{
    build_script, lockfile,
    miette::{self, Context as _, IntoDiagnostic as _},
};

fn main() -> miette::Result<()> {
    build_script::rerun_on_script_change();
    lockfile::assert_deps_consistency(&lockfile::load_workspace()?)?;
    let output_dir = chipbox_build::fs::ts_bindings_output()
        .into_diagnostic()
        .wrap_err("chipbox-solid-render")?;
    std::fs::create_dir_all(&output_dir)
        .into_diagnostic()
        .wrap_err("create dir all")?;
    println!("cargo:rustc-env=TS_EXPORT_DIR={}", output_dir.display());
    Ok(())
}
