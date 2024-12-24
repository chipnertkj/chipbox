#[derive(
    Debug,
    Clone,
    Copy,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
    derive_more::IntoIterator,
    derive_more::AsRef,
    derive_more::AsMut,
    derive_more::Index,
    derive_more::IndexMut,
    derive_more::Mul,
    derive_more::MulAssign,
)]
pub struct Block<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> Block<T, N> {
    pub fn new(frames: [T; N]) -> Self {
        Self::from(frames)
    }
}
