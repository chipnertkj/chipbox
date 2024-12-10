use crate::css::color::CssColor;
use frequency_label::FrequencyLabels;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

mod frequency_label;

pub use frequency_label::FrequencyLabelId;

/// A scale that defines the possible frequencies in a pattern.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrequencyScale {
    /// A short name representing the frequency scale.
    pub name: String,
    /// A color used to visually distinguish the frequency scale in the editor.
    pub color: CssColor,
    /// The available frequencies in the scale.
    pub labels: FrequencyLabels,
}

slotmap::new_key_type! {
    /// A unique identifier for a [`FrequencyScale`].
    pub struct FrequencyScaleId;
}

/// A container for [`FrequencyScale`] instances available in a song.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FrequencyScales {
    patterns: SlotMap<FrequencyScaleId, FrequencyScale>,
}
