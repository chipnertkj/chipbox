use serde::{Deserialize, Serialize};

#[derive(
    Debug, PartialEq, PartialOrd, Clone, Copy, Eq, Ord, Serialize, Deserialize,
)]
pub struct PatternId(pub u16);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SynthChannel {
    pub name: String,
    pub patterns: Vec<PatternId>,
    pub slots: Vec<PatternId>,
    pitch_count: u32,
    layer_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub layers: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub length: u8,
    pub pitch: u32,
}
