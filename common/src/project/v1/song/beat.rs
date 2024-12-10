//! Types for time measurement based on a [beat](https://en.wikipedia.org/wiki/Beat_(music)).

use serde::{Deserialize, Serialize};

/// A number representing a count of [beats](https://en.wikipedia.org/wiki/Beat_(music)).
#[derive(
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    derive_more::Not,
    derive_more::Add,
    derive_more::From,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct BeatCount(pub u32);

/// A floating-point distance in [beats](https://en.wikipedia.org/wiki/Beat_(music)) relative to an arbitrary point.
#[derive(
    PartialEq,
    PartialOrd,
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    derive_more::Add,
    derive_more::From,
    derive_more::AsRef,
    derive_more::AsMut,
)]
pub struct BeatDistance(pub f64);

impl BeatDistance {
    delegate::delegate! {
        to self.0 {
            /// Clamp the beat distance to a given range.
            /// Uses [`f64::clamp`].
            #[into]
            pub fn clamp(self, #[into] min: impl Into<f64>, #[into] max: impl Into<f64>) -> Self;
        }
    }
}
