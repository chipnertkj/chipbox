//! A channel on the timeline for producing sound
//! using notes, patterns and instrument node graphs.

use crate::css::color::CssColor;
use control_parameter::ControlParameters;
use frequency_scale::FrequencyScales;
use pattern::Patterns;
use preset::Presets;
use serde::{Deserialize, Serialize};
use timeline::Timeline;

mod control_parameter;
mod frequency_scale;
mod instrument;
mod pattern;
mod preset;
mod timeline;

pub use control_parameter::{ControlParameter, ControlParameterId};
pub use frequency_scale::{FrequencyLabelId, FrequencyScale, FrequencyScaleId};
pub use instrument::Instrument;
pub use pattern::{ControlPoint, Coordinate, Note, Pattern, PatternId};
pub use preset::{Preset, PresetId};

/// An audio channel that organizes instruments and notes
/// in order to produce sound.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentChannel {
    /// A short name describing the channel.
    pub name: String,
    /// The color used to visually distinguish the channel in the timeline.
    pub color: CssColor,

    /// The instrument used for audio rendering.
    pub instrument: Instrument,

    /// The timeline of an instrument channel.
    /// Defines the order in which [`Pattern`]s are played.
    /// Patterns may be reused across the timeline.
    pub timeline: Timeline,

    /// [`Pattern`]s for use across the timeline of the channel.
    /// Holds notes and parameters used for rendering.
    /// Patterns may be reused across the timeline.
    pub patterns: Patterns,

    /// [`Preset`]s for the instrument used by this channel.
    /// Each [`Pattern`] has a preset assigned that is used to render its notes.
    /// Presets can be reused across patterns.
    pub presets: Presets,

    /// [`FrequencyScale`]s exposed to the patterns in this channel.
    /// Every [`Pattern`] has a frequency scale assigned that is used to
    /// determine the frequency of its notes.
    /// Frequency scales can be reused across patterns.
    pub frequency_scales: FrequencyScales,

    /// [`ControlParameter`]s exposed to the patterns and their notes in this channel.
    /// Every note in a pattern can define [`ControlPoint`]s that affect the
    /// value of the parameter over the duration of the note.
    pub control_parameters: ControlParameters,
}
