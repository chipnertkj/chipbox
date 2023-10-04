pub mod synth;
pub use self::synth::SynthChannel;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Channels {
    pub synth: Vec<SynthChannel>,
}
