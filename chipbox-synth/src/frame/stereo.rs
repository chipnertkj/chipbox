use crate::frame::Frame;

pub type StereoFrame<SampleT> = Frame<SampleT, 2>;

impl<SampleT> StereoFrame<SampleT> {
    pub const LEFT_IDX: usize = 0;
    pub const RIGHT_IDX: usize = 1;

    pub fn left(&self) -> &SampleT {
        &self[Self::LEFT_IDX]
    }

    pub fn left_mut(&mut self) -> &mut SampleT {
        &mut self[Self::LEFT_IDX]
    }

    pub fn right(&self) -> &SampleT {
        &self[Self::RIGHT_IDX]
    }

    pub fn right_mut(&mut self) -> &mut SampleT {
        &mut self[Self::RIGHT_IDX]
    }
}
