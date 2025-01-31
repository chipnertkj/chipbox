//! Configuration file schema.

use miette::{Context as _, IntoDiagnostic as _, miette};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

/// Tool configuration. Defines what items to watch and build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Output configuration.
    pub output: Output,
    /// Bin targets to watch.
    pub bin: HashMap<String, Bin>,
    /// Hot lib targets to watch.
    pub hot: HashMap<String, BuildItem>,
}

impl Config {
    /// Verify that the paths in the config are valid.
    pub fn validate(&self) -> miette::Result<()> {
        self.bin.iter().try_for_each(|(name, bin)| {
            bin.validate()
                .wrap_err_with(|| format!("invalid bin `{}`", name))
        })?;
        self.hot.iter().try_for_each(|(name, item)| {
            item.validate()
                .wrap_err_with(|| format!("invalid hot lib `{}`", name))
        })?;
        Ok(())
    }
}

/// Tool output configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    /// Path to the output directory for the tool.
    pub path: PathBuf,
}

/// Bin target to watch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bin {
    /// Flattened build item configuration.
    #[serde(flatten)]
    pub build_item: BuildItem,
    /// Run the bin after building.
    /// Only runs it once per build - will not restart.
    pub run: bool,
}

impl Bin {
    /// Validate the bin.
    ///
    /// Ensures that the build item is valid.
    /// See [`BuildItem::validate`] for more details.
    fn validate(&self) -> miette::Result<()> {
        self.build_item.validate()
    }
}

/// Cargo crate to build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildItem {
    /// Path to the crate to build.
    pub path: PathBuf,
    /// Enabled features.
    #[serde(default)]
    pub enabled_features: Vec<String>,
    /// Whether to disable default features.
    #[serde(default = "BuildItem::default_default_features")]
    pub default_features: bool,
}

impl BuildItem {
    /// Default value for `default_features`.
    pub fn default_default_features() -> bool {
        true
    }

    /// Validate the build item.
    ///
    /// Ensures that the path is relative to the working directory and that the
    /// crate manifest file exists.
    fn validate(&self) -> miette::Result<()> {
        if self.path.is_absolute() {
            Err(miette!(
                help = "use paths relative to the working directory",
                "path is absolute",
            ))
        } else {
            let working_dir = std::env::current_dir()
                .into_diagnostic()
                .wrap_err("get working dir")?;
            let item_dir = working_dir.join(&self.path);
            let manifest_path = item_dir.join("Cargo.toml");
            if !manifest_path.exists() {
                Err(miette!(
                    help = "path does not exist",
                    "ensure you're in the right directory",
                ))
            } else {
                Ok(())
            }
        }
    }
}
