use crate::css::color::CssColor;
use serde::{Deserialize, Serialize};

/// An instrument in the song.
/// Reacts to notes and automation.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Instrument {
    /// A short name representing the instrument.
    pub name: String,
    /// A color used to visually distinguish the instrument in the timeline.
    pub color: CssColor,
}
