use goblin::Object;
use miette::Diagnostic;
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(thiserror::Error, Debug, Diagnostic)]
pub enum CheckerError<'path> {
    #[error("unsupported binary format")]
    #[diagnostic(code(dynlib_checker::unsupported_format))]
    UnsupportedFormat,
    #[error("failed to read library at {path}")]
    #[diagnostic(code(dynlib_checker::io_error))]
    FileError {
        path: &'path Path,
        #[source]
        e: std::io::Error,
    },
    #[error("failed to parse binary at {path}")]
    #[diagnostic(code(dynlib_checker::parse_error))]
    GoblinParseError {
        path: &'path Path,
        #[source]
        e: goblin::error::Error,
    },
    #[error("failed to parse json at {path}")]
    #[diagnostic(code(dynlib_checker::parse_error))]
    JsonParseError {
        path: &'path Path,
        #[source]
        e: serde_json::Error,
    },
}

#[derive(Debug, Serialize, Deserialize)]
struct SymbolCache {
    symbols: FxHashSet<String>,
    timestamp: std::time::SystemTime,
}

/// Checks for removed symbols in dynamic libraries
#[derive(Debug)]
pub struct DynlibChecker {
    lib_path: PathBuf,
    cache_path: PathBuf,
}

impl DynlibChecker {
    /// Create a new checker that stores its cache alongside the library
    pub fn new(lib_path: PathBuf) -> Self {
        let cache_path = lib_path.with_extension("chipbox-dev_symbols.json");
        Self {
            lib_path,
            cache_path,
        }
    }

    /// Check if a cache file exists for this library
    pub fn has_cache(&self) -> bool {
        self.cache_path.exists()
    }

    /// Extract symbols from a dynamic library
    async fn extract_symbols(&self) -> Result<FxHashSet<String>, CheckerError> {
        let buffer = fs::read(&self.lib_path)
            .await
            .map_err(|e| CheckerError::FileError {
                path: &self.lib_path,
                e,
            })?;

        let mut symbols = FxHashSet::default();

        match Object::parse(&buffer).map_err(|e| CheckerError::GoblinParseError {
            path: &self.lib_path,
            e,
        })? {
            Object::Elf(elf) => {
                // For Linux .so files
                for sym in elf.dynsyms.iter() {
                    if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
                        // We want symbols that are NOT imports
                        if !sym.is_import() {
                            symbols.insert(name.to_string());
                        }
                    }
                }
            }
            Object::Mach(mach) => {
                // For macOS .dylib files
                match mach {
                    goblin::mach::Mach::Binary(macho) => {
                        for (name, nlist) in macho.symbols().flatten() {
                            // Skip undefined (imported) symbols
                            if !nlist.is_undefined() {
                                symbols.insert(name.to_string());
                            }
                        }
                    }
                    goblin::mach::Mach::Fat(_) => {
                        return Err(CheckerError::UnsupportedFormat);
                    }
                }
            }
            Object::PE(pe) => {
                // For Windows .dll files
                for export in pe.exports {
                    if let Some(name) = export.name {
                        symbols.insert(name.to_string());
                    }
                }
            }
            _ => return Err(CheckerError::UnsupportedFormat),
        }

        Ok(symbols)
    }

    async fn load_cached_symbols(&self) -> Result<Option<SymbolCache>, CheckerError> {
        if !self.cache_path.exists() {
            return Ok(None);
        }
        let content =
            fs::read_to_string(&self.cache_path)
                .await
                .map_err(|e| CheckerError::FileError {
                    path: &self.cache_path,
                    e,
                })?;
        Ok(Some(serde_json::from_str(&content).map_err(|e| {
            CheckerError::JsonParseError {
                path: &self.cache_path,
                e,
            }
        })?))
    }

    async fn save_symbols(&self, symbols: FxHashSet<String>) -> Result<(), CheckerError> {
        let cache = SymbolCache {
            symbols,
            timestamp: std::time::SystemTime::now(),
        };

        let content =
            serde_json::to_string_pretty(&cache).map_err(|e| CheckerError::JsonParseError {
                path: &self.cache_path,
                e,
            })?;
        fs::write(&self.cache_path, content)
            .await
            .map_err(|e| CheckerError::FileError {
                path: &self.cache_path,
                e,
            })?;
        Ok(())
    }

    /// Check if any symbols were removed from the library
    /// Returns true if rebuilds are needed (symbols were removed)
    pub async fn check_for_removals(&self) -> Result<bool, CheckerError> {
        let old_symbols = self
            .load_cached_symbols()
            .await?
            .map(|cache| cache.symbols)
            .unwrap_or_default();

        let new_symbols = Self::extract_symbols(self).await?;

        // Check if any old symbols are missing from new set
        let has_removals = old_symbols
            .iter()
            .any(|old_sym| !new_symbols.contains(old_sym));

        // Only save new state if this is the first check (no cache) or if we detected changes
        if !self.has_cache() || has_removals {
            tracing::debug!("saving new symbol state");
            self.save_symbols(new_symbols).await?;
        }

        Ok(has_removals)
    }

    /// Clear the symbol cache
    pub async fn clear_cache(&self) -> Result<(), CheckerError> {
        if self.cache_path.exists() {
            fs::remove_file(&self.cache_path)
                .await
                .map_err(|e| CheckerError::FileError {
                    path: &self.cache_path,
                    e,
                })?;
        }
        Ok(())
    }
}
