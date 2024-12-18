#[derive(clap::Parser, Debug)]
pub(super) struct Args {
    /// Enable verbose logging.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
