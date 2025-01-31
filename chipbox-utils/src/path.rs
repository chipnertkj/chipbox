use miette::{Context as _, IntoDiagnostic as _};
use std::path::Path;

/// Display a path as a [`String`] while limiting the number of ancestors visible.
pub fn display_limit_ancestors(path: &Path, max_ancestors: usize) -> miette::Result<String> {
    let absolute = std::path::absolute(path)
        .into_diagnostic()
        .wrap_err("get absolute path")?;
    let mut ancestors = absolute.ancestors();
    let mut path_str = ancestors
        .take(max_ancestors)
        .map(|p| p.file_name().unwrap_or(p.as_os_str()).to_string_lossy())
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<Vec<_>>()
        .join("/");
    if ancestors.next().is_some() {
        path_str = format!(".../{path_str}");
    }
    Ok(path_str)
}
