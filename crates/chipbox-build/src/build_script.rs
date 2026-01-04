use std::sync::OnceLock;

/// The type of rerun function that has been called.
#[derive(Debug, Clone, Copy)]
enum Rerun {
    /// Rerun if the lockfile changes.
    Lockfile,
    /// Rerun if the local `build.rs` file changes.
    Script,
}

impl Rerun {
    fn emit(self) {
        let rhs = match self {
            Self::Lockfile => "../Cargo.lock",
            Self::Script => "build.rs",
        };
        println!("cargo:rerun-if-changed={rhs}");
    }
}

/// This is used to track which rerun functions have been called.
static _RERUN: OnceLock<Rerun> = OnceLock::new();

fn set_rerun(rerun: Rerun) {
    _RERUN.try_insert(rerun).expect("rerun already set");
    rerun.emit();
}

/// Rerun the build script if the lockfile changes.
/// This emits the `cargo:rerun-if-changed` directive to the build script.
///
/// ## Panics
/// - A `rerun_on_*` function has already been called.
pub fn rerun_on_lockfile_change() {
    set_rerun(Rerun::Lockfile);
}

/// Rerun the build script if the local `build.rs` file changes.
/// This emits the `cargo:rerun-if-changed` directive to the build script.
pub fn rerun_on_script_change() {
    set_rerun(Rerun::Script);
}

#[derive(Debug, derive_more::From, derive_more::Into, derive_more::Display)]
#[display("{} toolchain", _0)]
pub struct Toolchain(pub String);

impl Toolchain {
    #[must_use]
    pub fn from_env() -> Self {
        toolchain()
    }

    #[must_use]
    pub fn is_msvc(&self) -> bool {
        matches!(self.0.as_str(), "msvc")
    }
}

/// Check for MSVC toolchain.
/// # Panics
/// - If `CARGO_CFG_TARGET_ENV` is not set.
#[must_use]
pub fn toolchain() -> Toolchain {
    std::env::var("CARGO_CFG_TARGET_ENV")
        .expect("build script env")
        .into()
}
