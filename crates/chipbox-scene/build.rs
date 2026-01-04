use chipbox_build::miette::{self as miette, Context, IntoDiagnostic};

#[tokio::main]
async fn main() -> miette::Result<()> {
    chipbox_build::build_script::rerun_on_script_change();
    let output_dir = chipbox_build::fs::ts_bindings_output()
        .await
        .into_diagnostic()
        .wrap_err("chipbox-solid-render")?;
    tokio::fs::create_dir_all(&output_dir)
        .await
        .into_diagnostic()
        .wrap_err("create dir all")?;
    println!("cargo:rustc-env=TS_EXPORT_DIR={}", output_dir.display());
    Ok(())
}
