//! Operations on the workspace lockfile.

use std::borrow::Cow;

use cargo_lock::{Dependency, Lockfile, Package, package::SourceKind};
use miette::{Context as _, IntoDiagnostic as _};

/// Find a package with the given name in the lockfile.
///
/// ## Errors
/// - Unable to find a package with the given name.
pub fn pkg<'a>(lockfile: &'a Lockfile, name: &str) -> miette::Result<&'a Package> {
    lockfile
        .packages
        .iter()
        .find(|pkg| pkg.name.as_str() == name)
        .ok_or_else(|| miette::miette!("unable to find package `{name}`"))
}

/// Find a package that matches the given dependency in the lockfile.
///
/// ## Errors
/// - Unable to find a package that matches the given dependency.
pub fn pkg_from<'a>(lockfile: &'a Lockfile, dep: &'a Dependency) -> miette::Result<&'a Package> {
    lockfile
        .packages
        .iter()
        .find(|pkg| dep.matches(pkg))
        .ok_or_else(|| miette::miette!("unable to find matching package"))
}

/// Find a dependency with the given name in the package.
///
/// ## Errors
/// - Unable to find a dependency with the given name.
pub fn dep<'a>(pkg: &'a Package, name: &str) -> miette::Result<&'a Dependency> {
    pkg.dependencies
        .iter()
        .find(|pkg| pkg.name.as_str() == name)
        .ok_or_else(|| miette::miette!("unable to find dependency `{name}`"))
}

/// Find the self package in the lockfile.
/// This is the package that this function is being called from at runtime.
///
/// ## Panics
/// - Could not get path to self package.
///
/// ## Errors
/// - Unable to find package that matches the path.
pub fn self_pkg(lockfile: &Lockfile) -> miette::Result<&Package> {
    LockfilePath::this_pkg().find_pkg(lockfile)
}

/// A forward slash `/` separated path to a package in the lockfile.
///
/// Example: `"chipbox-render/vello"` yields the package `vello`.
/// If there are multiple packages with the name `vello`,
/// this always returns only the one that `chipbox-render` depends on directly.
///
/// The first component is always a package defined in a lockfile.
/// The following components are dependencies of the subsequent packages.
#[derive(Debug, PartialEq, Eq)]
pub struct LockfilePath<'a> {
    s: Cow<'a, str>,
}

impl AsRef<str> for LockfilePath<'_> {
    fn as_ref(&self) -> &str {
        self.s.as_ref()
    }
}

impl<'a, S: Into<Cow<'a, str>>> From<S> for LockfilePath<'a> {
    fn from(s: S) -> Self {
        Self::new(s)
    }
}

impl<'a> LockfilePath<'a> {
    /// Create a new lockfile path from the given string.
    #[must_use]
    pub fn new(path: impl Into<Cow<'a, str>>) -> Self {
        Self { s: path.into() }
    }

    /// Get path to the package this function is being called from.
    ///
    /// ## Panics
    /// - `CARGO_PKG_NAME` environment variable is not set.
    #[must_use]
    pub fn this_pkg() -> Self {
        Self::new(std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME is not set"))
    }

    /// Recursively find a package with the given path.
    ///
    /// ## Errors
    /// - Path is empty.
    /// - Unable to find self package.
    /// - Unable to find a dependency or package that matches the path.
    pub fn find_pkg<'b>(&self, lockfile: &'b Lockfile) -> miette::Result<&'b Package> {
        let mut components = self.components();
        let mut pkg = pkg(
            lockfile,
            components
                .next()
                .ok_or_else(|| miette::miette!("path is empty"))?,
        )
        .wrap_err("find root pkg")?;
        for component in components {
            let dep = dep(pkg, component)
                .wrap_err_with(|| format!("find dep `{component}` in `{}`", pkg.name))?;
            pkg = pkg_from(lockfile, dep)
                .wrap_err_with(|| format!("get pkg `{component}` from dep `{}`", dep.name))?;
        }
        Ok(pkg)
    }

    /// Reformat the path as a pretty string.
    /// Strips any leading or trailing slashes and joins the components with a forward slash `/`.
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

/// Read the lockfile from the workspace directory.
///
/// ## Panics
/// - `CARGO_WORKSPACE_DIR` environment variable is not set.
///
/// ## Errors
/// - Unable to load lockfile.
pub fn load_workspace() -> miette::Result<Lockfile> {
    let path = format!(
        "{}/Cargo.lock",
        std::env::var("CARGO_WORKSPACE_DIR").expect("CARGO_WORKSPACE_DIR is not set")
    );
    let lockfile = Lockfile::load(&path)
        .into_diagnostic()
        .wrap_err(path)
        .wrap_err("failed to load lockfile")?;
    Ok(lockfile)
}

/// Find dependencies of a package that are in the local filesystem.
///
/// ## Errors
/// - Unable to find a dependency or package that matches the path.
///
/// ## Panics
/// - No source field on a dependency or its package.
pub fn find_local_deps<'a>(
    lockfile: &'a Lockfile,
    path: impl Into<LockfilePath<'a>>,
) -> miette::Result<Vec<&'a Package>> {
    let path = path.into();
    let pkg = path.find_pkg(lockfile).wrap_err("find pkg")?;
    let dep_pkgs: Vec<_> = pkg
        .dependencies
        .iter()
        // Some dependency specifiers may have a source field.
        // Filter those out if they are not a local path.
        .filter(|dep| {
            dep.source
                .as_ref()
                .is_none_or(|source| *source.kind() == SourceKind::Path)
        })
        .map(|dep| pkg_from(lockfile, dep).wrap_err("pkg from dep"))
        .collect::<Result<_, _>>()
        .wrap_err("filter map deps")?;
    let local_pkgs = dep_pkgs
        .into_iter()
        // Any packages that are not a local path are not local dependencies.
        .filter(|pkg| {
            pkg.source
                .as_ref()
                .map(|source| *source.kind() == SourceKind::Path)
                // If there is no source field, I guess we have no idea.
                .expect("no source field")
        })
        .collect();
    Ok(local_pkgs)
}
