#![feature(once_cell_try_insert)]

#[cfg(feature = "lockfile")]
pub use cargo_lock;
pub use miette;

#[cfg(feature = "build-script")]
pub mod build_script;
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "lockfile")]
pub mod lockfile;

#[derive(Default, Debug, Clone, Copy)]
pub enum CargoProfile {
    #[default]
    Dev,
    Release,
}

impl CargoProfile {
    #[must_use]
    pub const fn is_release(self) -> bool {
        matches!(self, Self::Release)
    }

    #[must_use]
    pub const fn target_folder(self) -> &'static str {
        match self {
            Self::Dev => "debug",
            Self::Release => "release",
        }
    }
}
