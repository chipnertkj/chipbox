//! On-the-fly instrument parameter customization.

use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

/// A collection of values fed to an instrument's parameters.
///
/// Enables customizing the sound of an instrument without changing
/// the underlying node graph.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Preset {}

slotmap::new_key_type! {
    /// A unique identifier for a [`Preset`].
    pub struct PresetId;
}

/// A container for [`Preset`] instances available in a song.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Presets {
    presets: SlotMap<PresetId, Preset>,
}
