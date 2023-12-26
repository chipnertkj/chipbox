mod mod_layer;

pub use self::mod_layer::{ModLayer, ModLayerMeta};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Pattern {
    length: u32,
    notes: Vec<Note>,
    mod_layers: Vec<ModLayer>,
}

impl Pattern {
    pub fn set_pitch_count(&mut self, pitch_count: u32) {
        self.notes
            .retain(|x| x.pitch < pitch_count);
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Note {
    pub length: u32,
    pub pitch: u32,
}
