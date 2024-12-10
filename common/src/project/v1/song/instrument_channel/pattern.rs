use super::{FrequencyScaleId, PresetId};
use crate::project::latest::song::beat::BeatCount;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

mod note;

pub use note::{ControlPoint, Coordinate, Note};

/// A collection of notes organized into a pattern.
/// Used for audio rendering.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Pattern {
    notes: Vec<Note>,
    /// The number of beats in the pattern.
    /// The speed at which the pattern is played is defined by the song's
    /// beats-per-minute value.
    width: BeatCount,
    /// Identifier of the pattern's frequency scale.
    /// The available scales are defined by the channel.
    frequency_scale: FrequencyScaleId,
    /// The identifier of the instrument preset used for playback.
    /// The available presets are defined by the channel.
    preset: PresetId,
}

impl Pattern {
    /// Creates an empty pattern with the given paraneters.
    pub fn new(width: BeatCount, frequency_scale: FrequencyScaleId, preset: PresetId) -> Self {
        Self {
            notes: vec![],
            width,
            frequency_scale,
            preset,
        }
    }

    /// Return a slice containing all notes in the pattern.
    pub fn notes(&self) -> &[Note] {
        &self.notes
    }

    /// Resizes the pattern to the given width.
    ///
    /// Each note is validated with the new parameters using
    /// [`Self::validate_note`].
    /// Invalid notes are discarded.
    pub fn set_width(&mut self, width: BeatCount) {
        // Set new width first.
        self.width = width;
        // Then validate each note.
        self.notes = self
            .notes
            .drain(..)
            // Validate notes using new parameters.
            .filter_map(|note| Self::validate_note(note, width))
            .collect();
    }

    /// Add a [`Note`] to the pattern.
    ///
    /// The note is validated using [`Self::validate_note`].
    /// If it turns out to be invalid, the function returns [`None`].
    pub fn add_note(&mut self, note: Note) -> Option<()> {
        Self::validate_note(note, self.width)
            // Add note if valid.
            .map(|note| self.notes.push(note))
    }

    /// Modifies a [`Note`] so that it fits within the bounds of a
    /// pattern of given width.
    /// - The start [`Coordinate`] of the note must be >= 0.0
    /// - The end [`Coordinate`] of the note must be <= `pattern_width`
    ///
    /// Returns [`None`] if a valid note cannot exist given the requirements.
    fn validate_note(note: Note, pattern_width: BeatCount) -> Option<Note> {
        note.clip(0.0, pattern_width.0)
    }
}

slotmap::new_key_type! {
    /// A unique identifier for a [`Pattern`].
    pub struct PatternId;
}

/// A container for [`Pattern`] instances available in a song.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Patterns {
    patterns: SlotMap<PatternId, Pattern>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::v1::song::instrument_channel::FrequencyLabelId;
    use note::Coordinate;

    #[test]
    fn set_width() {
        let label_id = FrequencyLabelId::default();
        let mut pattern = Pattern::new(3.into(), FrequencyScaleId::default(), PresetId::default());
        pattern.add_note(Note::new_flat(Coordinate::new(0.0, label_id), 1.0).unwrap());
        pattern.add_note(Note::new_flat(Coordinate::new(-1.0, label_id), 4.0).unwrap());
        pattern.add_note(Note::new_flat(Coordinate::new(2.0, label_id), 1.0).unwrap());
        assert_eq!(pattern.notes().len(), 3);
        pattern.set_width(1.into());
        assert_eq!(pattern.notes().len(), 2);
    }
}
