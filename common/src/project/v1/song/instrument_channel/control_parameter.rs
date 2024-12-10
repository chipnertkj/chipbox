use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

/// A parameter that can be controlled by an instrument.
///
/// The value of a control parameter is stored per-note by control points across
/// that note.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct ControlParameter {
    /// A short name describing the control parameter.
    pub name: String,
}

slotmap::new_key_type! {
    /// A unique identifier for a [`ControlParameter`].
    pub struct ControlParameterId;
}

/// A container for [`ControlParameter`] instances available in a song.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ControlParameters {
    patterns: SlotMap<ControlParameterId, ControlParameter>,
}
