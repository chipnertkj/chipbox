use crate::project::v1::song::beat::BeatDistance;
use itertools::Itertools as _;
use serde::{Deserialize, Serialize};

mod control_point;
mod cooordinate;

pub use control_point::ControlPoint;
pub use cooordinate::Coordinate;

/// A single note in a pattern.
///
/// A note is described by a sorted set of coordinates.
/// Each [`Coordinate`] is a position on the x axis of a pattern, as well as the
/// frequency label, based on the pattern's frequency scale.
///
/// A note is ensured to have at least two coordinates.
/// Coordinates in the same position on the x axis are collapsed.
/// The frequency label of the left-most coordinate is used.
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    /// Coordinates that define a note.
    coords: Vec<Coordinate>,
    /// Control points that enable per-note automation.
    control_points: Vec<ControlPoint>,
}

impl Note {
    /// Creates a new [`Note`] from a single [`Coordinate`] and a width.
    pub fn new_flat(start_coord: Coordinate, width: impl Into<BeatDistance>) -> Option<Self> {
        let end_coord = Coordinate {
            x: start_coord.x + width.into(),
            ..start_coord
        };
        Self::new([start_coord, end_coord])
    }

    /// Creates a new [`Note`] from an iterable collection of [`Coordinate`]
    /// items.
    ///
    /// The coordinates are sorted and deduplicated before attempting to
    /// construct the note.
    ///
    /// If the processed coordinate set has less than two unique coordinates,
    /// the note is considered invalid.
    pub fn new(coords: impl IntoIterator<Item = Coordinate>) -> Option<Self> {
        // Sort and dedup coords.
        let coords = coords
            .into_iter()
            // Sort across the x axis.
            .sorted_by(|a, b| a.x.partial_cmp(&b.x).unwrap())
            // Remove points that are on the same x position.
            .dedup_by(|a, b| a.x == b.x)
            .collect::<Vec<_>>();

        // Ensure there are at least two coords.
        if coords.len() >= 2 {
            // Note is valid.
            Some(Self {
                coords,
                control_points: vec![],
            })
        } else {
            // Note had less than two unique points.
            // It doesn't have a length and is considered invalid.
            None
        }
    }

    /// Get the coordinates that define a [`Note`].
    ///
    /// The returned coordinates are ensured to be sorted by their x position.
    /// It is also ensured that there are at least two of them and that they are
    /// not on the same x position.
    pub fn coords(&self) -> &[Coordinate] {
        &self.coords
    }

    /// Clip the [`Note`] to the given bounds on the x axis.
    ///
    /// The final note is constructed from the clipped coordinates
    /// using [`Note::new`].
    ///
    /// # Panics
    /// Panics if any of the
    pub fn clip(self, min_x: impl Into<f64>, max_x: impl Into<f64>) -> Option<Self> {
        let min_x = min_x.into();
        let max_x = max_x.into();
        let coords = self.coords.into_iter().map(|mut coord| {
            coord.x = coord.x.clamp(min_x, max_x);
            coord
        });
        Self::new(coords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::v1::song::instrument_channel::frequency_scale::FrequencyLabelId;

    /// Ensure we can construct a unit note without issues.
    /// The note should have two coordinates, one at x 0.0 and one at x 1.0.
    #[test]
    fn unit_note() {
        let label_id = FrequencyLabelId::default();
        let base_coord = Coordinate::new(0.0, label_id);
        let expected = vec![base_coord, Coordinate::new(1.0, label_id)];
        let note = Note::new_flat(base_coord, 1.0).unwrap();
        assert_eq!(note.coords, expected);
    }

    /// Ensure we cannot construct a note with less than two unique coordinates.
    #[test]
    fn invalid_note() {
        let label_id = FrequencyLabelId::default();
        let base_coord = Coordinate::new(0.0, label_id);
        let note = Note::new_flat(base_coord, 0.0);
        assert!(note.is_none());
    }
}
