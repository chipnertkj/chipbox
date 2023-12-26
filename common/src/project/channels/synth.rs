mod pattern;
mod tuning;

use self::pattern::{ModLayerMeta, Pattern};
use self::tuning::Tuning;

#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Clone,
    Copy,
    Eq,
    Ord,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct PatternId(pub u16);

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
/// An audio channel that defines its own synthesizer.
///
/// A `SynthChannel` is a set of `Patterns`.
/// More specifically, a synth channel is a sequence of IDs that refer to patterns,
/// which themselves are also stored by the channel.
/// Those ID items are called `Slot`s.
///
/// `Pattern`s are used to provide the synthesizer with note events.
/// They are loosely unique (users may define patterns that hold identical data),
/// but different slots can refer to the same pattern.
///
/// During rendering, the renderer travels through the slots in a linear fashion.
/// It then uses note data from the pattern that the slot at a given timestamp refers to.
/// Note data is then passed to the synthesizer node graph as note events.
///
/// The graph outputs a computed sample per each note at a given timestamp.
/// Those samples are mixed together to produce the final audio output.
pub struct SynthChannel {
    pub name: String,
    pub patterns: Vec<Pattern>,
    pub slots: Vec<PatternId>,
    mod_layers_meta: Vec<ModLayerMeta>,
    tuning: Tuning,
}

impl SynthChannel {
    pub fn new(name: String, tuning: Tuning) -> Self {
        Self {
            name,
            patterns: Vec::new(),
            slots: Vec::new(),
            tuning,
            mod_layers_meta: Vec::new(),
        }
    }

    pub fn mod_layers_meta(&self) -> &[ModLayerMeta] {
        &self.mod_layers_meta
    }

    pub fn mod_layers_meta_mut(&mut self) -> &mut [ModLayerMeta] {
        &mut self.mod_layers_meta
    }
}
