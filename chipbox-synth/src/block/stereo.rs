use crate::{block::Block, frame::StereoFrame};

pub type StereoBlock<SampleT, const FRAME_COUNT: usize> = Block<StereoFrame<SampleT>, FRAME_COUNT>;

impl<SampleT, const FRAME_COUNT: usize> StereoBlock<SampleT, FRAME_COUNT>
where
    SampleT: Copy,
{
    pub fn split_into(
        self,
        left: &mut Block<SampleT, FRAME_COUNT>,
        right: &mut Block<SampleT, FRAME_COUNT>,
    ) {
        self.as_frames()
            .iter()
            .enumerate()
            .for_each(|(frame_ix, frame)| {
                left.as_frames_mut()[frame_ix] = *frame.left();
                right.as_frames_mut()[frame_ix] = *frame.right();
            });
    }
}
