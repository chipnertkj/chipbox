use cargo_lock::{Dependency, Lockfile, Package};
use miette::{Context as _, IntoDiagnostic as _};

/// Rerun the build script if the lockfile changes.
/// This emits the `cargo:rerun-if-changed` directive to the build script.
pub fn set_rerun_on_change() {
    println!("cargo:rerun-if-changed=../Cargo.lock");
}

/// Find a package with the given name in the lockfile.
fn pkg<'lock>(lockfile: &'lock Lockfile, name: &str) -> miette::Result<&'lock Package> {
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

/// Find the self package in the lockfile.
///
/// ## Panics
/// - `CARGO_PKG_NAME` is not set.
fn self_pkg(lockfile: &Lockfile) -> miette::Result<&Package> {
    pkg(
        lockfile,
        &std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set"),
    )
    .wrap_err("find self pkg")
}

pub struct LockfilePath<'a> {
    s: &'a str,
}

impl<'a> From<&'a str> for LockfilePath<'a> {
    fn from(s: &'a str) -> Self {
        Self::new(s)
    }
}

impl<'a> LockfilePath<'a> {
    /// Create a new lockfile path from the given string.
    #[must_use]
    pub const fn new(path: &'a str) -> Self {
        Self { s: path }
    }

    /// Recursively find a package with the given path.
    ///
    /// ## Errors
    /// - Unable to find self package.
    /// - Unable to find a dependency or package that matches the path.
    pub fn find_pkg<'lock>(&self, lockfile: &'lock Lockfile) -> miette::Result<&'lock Package> {
        let mut pkg = self_pkg(lockfile).wrap_err("find self pkg")?;
        for component in self.components() {
            let dep = find_dep(pkg, component)
                .wrap_err_with(|| format!("find dep `{component}` in `{}`", pkg.name))?;
            pkg = pkg_from(lockfile, dep)
                .wrap_err_with(|| format!("get pkg `{component}` from dep `{}`", dep.name))?;
        }
        Ok(pkg)
    }

    /// Display the path as a string.
    #[must_use]
    pub fn display(&self) -> String {
        self.components().collect::<Vec<_>>().join("/")
    }

    /// Get the components of the path.
    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.s.trim_matches('/').split('/')
    }
}

/// Verify that package versions match for `left` and `right`.
///
/// ## Errors
/// - Version mismatch.
/// - Unable to find either package.
pub fn assert_versions_match<'left, 'right>(
    lockfile: &Lockfile,
    left: impl Into<LockfilePath<'left>>,
    right: impl Into<LockfilePath<'right>>,
) -> miette::Result<()> {
    let left = left.into();
    let right = right.into();
    let left_pkg = left.find_pkg(lockfile).wrap_err("find left pkg")?;
    let right_pkg = right.find_pkg(lockfile).wrap_err("find right pkg")?;
    if left_pkg.version != right_pkg.version {
        return Err(miette::miette!(
            "left: {}, right: {}",
            left_pkg.version,
            right_pkg.version
        ))
        .wrap_err(format!(
            "version mismatch between `{}` and `{}`",
            left.display(),
            right.display()
        ));
    }
    Ok(())
}

/// Read the lockfile.
///
/// ## Errors
/// - Unable to load lockfile.
pub fn read() -> miette::Result<Lockfile> {
    let path = format!("{}/Cargo.lock", env!("CARGO_WORKSPACE_DIR"));
    let lockfile = Lockfile::load(&path)
        .into_diagnostic()
        .wrap_err(path)
        .wrap_err("failed to load lockfile")?;
    Ok(lockfile)
}
