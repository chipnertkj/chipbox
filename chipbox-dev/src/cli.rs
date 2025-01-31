//! [`clap`] CLI interface.

/// CLI tool for hot-reloading chipbox.
#[derive(clap::Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Increase message verbosity (-v = debug, -vv = trace).
    #[arg(
        short = 'v',
        action = clap::ArgAction::Count,
        default_value_t,
        value_parser = clap::value_parser!(u8).range(0..=2)
    )]
    verbosity_raw: u8,
}

impl Args {
    /// Get verbosity level as [`Verbosity`] enum.
    pub fn verbosity(&self) -> Verbosity {
        self.verbosity_raw.into()
    }
}

/// Tracing verbosity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, derive_more::Display, Default)]
pub enum Verbosity {
    /// Normal verbosity.
    /// User-facing information.
    #[default]
    #[display("normal")]
    Normal,
    /// Debug verbosity.
    /// General diagnostic information.
    #[display("debug")]
    Debug {
        /// Whether this verbosity was set due to debug assertions.
        from_debug_assertions: bool,
    },
    /// Trace verbosity.
    /// Config dumps etc.
    #[display("trace")]
    Trace,
}

/// Convert [`u8`] to [`Verbosity`] enum.
impl From<u8> for Verbosity {
    fn from(verbosity: u8) -> Self {
        match verbosity {
            0 => Verbosity::Normal,
            1 => Verbosity::Debug {
                from_debug_assertions: false,
            },
            _ => Verbosity::Trace,
        }
    }
}

impl From<Verbosity> for u8 {
    fn from(verbosity: Verbosity) -> Self {
        match verbosity {
            Verbosity::Normal => 0,
            Verbosity::Debug { .. } => 1,
            Verbosity::Trace => 2,
        }
    }
}
