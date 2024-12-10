use crate::project::v1::song::{
    beat::BeatDistance, instrument_channel::frequency_scale::FrequencyLabelId,
};
use serde::{Deserialize, Serialize};

/// A positional coordinate that defines the placement of a note
/// across a pattern.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Coordinate {
    /// The horizontal position of a coordinate of a note, relative to the
    /// beginning of the pattern.
    pub x: BeatDistance,
    /// The vertical position of a coordinate of a note,
    /// relative to the lowest possible frequency label in a pattern.
    /// This is based on the pattern's frequency scale.
    pub freq_label_id: FrequencyLabelId,
}

impl Coordinate {
    /// Creates a new coordinate with a particular horizontal and vertical
    /// position.
    pub fn new(x: impl Into<BeatDistance>, freq_label_id: impl Into<FrequencyLabelId>) -> Self {
        Self {
            x: x.into(),
            freq_label_id: freq_label_id.into(),
        }
    }
}
