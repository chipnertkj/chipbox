use crate::frame::Frame;

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
    pub fn as_samples(&self) -> &[SampleT; FRAME_COUNT * CHANNEL_COUNT] {
        unsafe { std::mem::transmute(&self.data) }
    }

    pub fn as_samples_mut(&mut self) -> &mut [SampleT; FRAME_COUNT * CHANNEL_COUNT] {
        unsafe { std::mem::transmute(&mut self.data) }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_ref_safety() {
        let mut block = Block::new([Frame::from([0, 0]); 128]);
        block.as_samples_mut().iter_mut().for_each(|s| {
            *s += 1;
        });
        assert_eq!(
            block.as_samples().iter().sum::<i32>(),
            block.sample_count() as i32
        );
    }
}
