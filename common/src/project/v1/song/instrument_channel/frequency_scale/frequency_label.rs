use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

/// A label that denotes a frequency playable by an instrument.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub(super) struct FrequencyLabel {
    pub name: String,
    /// The frequency of the audio fed to the instrument in Hz.
    pub frequency: f64,
}

slotmap::new_key_type! {
    /// A unique identifier for a [`Pattern`].
    pub struct FrequencyLabelId;
}

/// A container for [`Pattern`] instances available in a song.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrequencyLabels {
    patterns: SlotMap<FrequencyLabelId, FrequencyLabel>,
}
