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
#[repr(transparent)]
pub struct Block<FrameT, const FRAME_COUNT: usize> {
    data: [FrameT; FRAME_COUNT],
}

impl<FrameT, const FRAME_COUNT: usize> Block<FrameT, FRAME_COUNT> {
    pub fn new(frames: [FrameT; FRAME_COUNT]) -> Self {
        Self::from(frames)
    }

    pub fn as_slice(&self) -> &[FrameT] {
        &self.data
    }
}
