use crate::project::v1::song::{beat::BeatDistance, instrument_channel::ControlParameterId};
use serde::{Deserialize, Serialize};

/// A control point that enables per-note automation.
/// Defines the value that a control parameter should
/// have on a note at a particular point across its length.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ControlPoint {
    /// The horizontal position of a control point of a note, relative to the
    /// coordinate closest to the beginning of the pattern.
    pub x: BeatDistance,
    /// The identifier of the parameter being controlled.
    pub parameter: ControlParameterId,
    /// The value that the parameter should be set to at this positiion.
    pub value: f64,
}
