use cargo_lock::{Dependency, Lockfile, Package};
use miette::{Context as _, IntoDiagnostic as _};

/// # Responsibilities:
/// - Verify `::wgpu` and `::vello::wgpu` versions match.
fn main() -> miette::Result<()> {
    rerun_on_lockfile_change();
    let lockfile = read_lockfile().wrap_err("failed to read lockfile")?;
    assert_wgpu_versions_match(&lockfile).wrap_err("failed to verify wgpu versions match")?;
    Ok(())
}

/// Find a package with the given name in the lockfile.
fn find_pkg<'lock>(lockfile: &'lock Lockfile, name: &str) -> miette::Result<&'lock Package> {
    lockfile
        .packages
        .iter()
        .find(|pkg| pkg.name.as_str() == name)
        .ok_or_else(|| miette::miette!("unable to find package `{name}`"))
}

/// Find a package that matches the given dependency in the lockfile.
fn pkg_from<'lock>(
    lockfile: &'lock Lockfile,
    dep: &'lock Dependency,
) -> miette::Result<&'lock Package> {
    lockfile
        .packages
        .iter()
        .find(|pkg| dep.matches(pkg))
        .ok_or_else(|| miette::miette!("unable to find matching package"))
}

/// Find a dependency with the given name in the package.
fn find_dep<'pkg>(pkg: &'pkg Package, name: &str) -> miette::Result<&'pkg Dependency> {
    pkg.dependencies
        .iter()
        .find(|pkg| pkg.name.as_str() == name)
        .ok_or_else(|| miette::miette!("unable to find dependency `{name}`"))
}

/// Verify that `::wgpu` and `::vello::wgpu` versions match.
fn assert_wgpu_versions_match(lockfile: &Lockfile) -> miette::Result<()> {
    let self_pkg_name = env!("CARGO_PKG_NAME");
    let self_pkg = find_pkg(lockfile, self_pkg_name)?;
    let wgpu_dep = find_dep(self_pkg, "wgpu").wrap_err("find crate::wgpu dep")?;
    let vello_dep = find_dep(self_pkg, "vello").wrap_err("find crate::vello dep")?;
    let vello_pkg = pkg_from(lockfile, vello_dep).wrap_err("get crate::vello pkg")?;
    let vello_wgpu_pkg = find_dep(vello_pkg, "wgpu").wrap_err("find vello::wgpu dep")?;
    assert_eq!(
        wgpu_dep.version, vello_wgpu_pkg.version,
        "version mismatch between `::wgpu` and `::vello::wgpu`"
    );
    Ok(())
}

/// Read the lockfile.
fn read_lockfile() -> miette::Result<Lockfile> {
    let path = format!("{}/Cargo.lock", env!("CARGO_WORKSPACE_DIR"));
    let lockfile = Lockfile::load(&path)
        .into_diagnostic()
        .wrap_err(path)
        .wrap_err("failed to load lockfile")?;
    Ok(lockfile)
}

/// Rerun the build script if the lockfile changes.
fn rerun_on_lockfile_change() {
    println!("cargo:rerun-if-changed=../Cargo.lock");
}
