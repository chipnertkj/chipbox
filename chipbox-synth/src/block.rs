use crate::frame::Frame;

mod stereo;

pub use stereo::StereoBlock;

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

    pub fn as_frames(&self) -> &[FrameT; FRAME_COUNT] {
        &self.data
    }

    pub fn as_frames_mut(&mut self) -> &mut [FrameT; FRAME_COUNT] {
        &mut self.data
    }
}

impl<SampleT, const FRAME_COUNT: usize, const CHANNEL_COUNT: usize>
    Block<Frame<SampleT, CHANNEL_COUNT>, FRAME_COUNT>
{
    pub const fn sample_count(&self) -> usize {
        FRAME_COUNT * CHANNEL_COUNT
    }

    pub const fn channel_count(&self) -> usize {
        CHANNEL_COUNT
    }
}

impl<FrameT, const FRAME_COUNT: usize> Default for Block<FrameT, FRAME_COUNT>
where
    FrameT: Default + Copy,
{
    fn default() -> Self {
        Self::new([FrameT::default(); FRAME_COUNT])
    }
}
