use miette::{Context as _, IntoDiagnostic as _};

async fn compile() -> miette::Result<()> {
    let path = crate::fs::cargo_workspace()
        .await
        .into_diagnostic()
        .wrap_err("find cargo workspace")?
        .join("portaudio");
    if !path
        .try_exists()
        .into_diagnostic()
        .wrap_err("check portaudio source")?
    {
        miette::bail!(
            help = "make sure to clone chipbox with --recurse-submodules",
            "portaudio source not found",
        )
    }
    todo!()
}
