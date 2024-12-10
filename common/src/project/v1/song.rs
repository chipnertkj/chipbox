//! Primary form of output for the program.

use instrument_channel::InstrumentChannel;
use meta::SongMeta;
use serde::{Deserialize, Serialize};

pub mod beat;
pub mod instrument_channel;
pub mod meta;

/// It contains the data needed to render the song, including the notes,
/// automation, and instrument channels, as well as some additional metadata.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Song {
    /// Additional context that describes the song but is not part of the audio rendering process.
    pub meta: SongMeta,
    /// The instrument channels that make up the song.
    pub instrument_channels: Vec<InstrumentChannel>,
}

impl Song {
    /// Create an empty song with the given metadata.
    pub fn new(meta: SongMeta) -> Self {
        Self {
            meta,
            instrument_channels: vec![],
        }
    }
}
