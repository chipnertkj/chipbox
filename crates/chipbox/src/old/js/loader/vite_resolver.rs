//! Vite module resolver for rquickjs.
//!
//! Resolves ES module specifiers to paths that can be fetched from the Vite dev server.

use rquickjs::{Ctx, Error, loader::Resolver};

/// Resolves module specifiers to Vite-compatible paths.
///
/// Vite transforms modules and serves them with special handling:
/// - Bare specifiers like `"solid-js"` -> pre-bundled in `node_modules/.vite/deps/`
/// - Relative imports `"./foo"` -> resolved relative to importer
/// - Absolute paths `"/src/main.tsx"` -> used as-is
#[derive(Default)]
pub struct ViteResolver;

impl ViteResolver {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Resolver for ViteResolver {
    fn resolve(&mut self, _ctx: &Ctx<'_>, base: &str, name: &str) -> Result<String, Error> {
        tracing::trace!("resolve {name} in {base}");
        // Native chipbox:* modules are handled by `ModuleLoader`, not `ViteLoader`.
        if name.starts_with("/@id/chipbox:") {
            tracing::trace!("resolving chipbox:* module {name}");
            return Ok(name
                .strip_prefix("/@id/")
                .expect("checked prefix")
                .to_string());
        }

        let resolved = if name.starts_with("./") || name.starts_with("../") {
            // Relative import - resolve against base's directory
            resolve_relative(base, name)
        } else if name.starts_with('/') {
            // Absolute path - use as-is
            name.to_string()
        } else {
            // Bare specifier (e.g., "solid-js", "chipbox-solid-render")
            // Vite pre-bundles these into node_modules/.vite/deps/
            // The exact path depends on how Vite transforms them
            // For now, we'll let Vite handle the resolution by prefixing with /@id/
            // which tells Vite to resolve it as a bare import
            format!("/@id/{name}")
        };

        tracing::trace!("resolved {name} to {resolved} in {base}");
        Ok(resolved)
    }
}

/// Resolve a relative path against a base path.
fn resolve_relative(base: &str, relative: &str) -> String {
    // Get the directory of the base path
    let base_dir = base.rfind('/').map_or("", |idx| &base[..idx]);

    // Split relative path into components
    let mut components: Vec<&str> = if base_dir.is_empty() {
        Vec::new()
    } else {
        base_dir.split('/').filter(|s| !s.is_empty()).collect()
    };

    for part in relative.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                components.pop();
            }
            other => components.push(other),
        }
    }

    if components.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", components.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_relative() {
        assert_eq!(resolve_relative("/src/main.tsx", "./App"), "/src/App");
        assert_eq!(
            resolve_relative("/src/components/Button.tsx", "../utils/helpers"),
            "/src/utils/helpers"
        );
        assert_eq!(
            resolve_relative("/src/main.tsx", "./lib/foo"),
            "/src/lib/foo"
        );
    }
}
