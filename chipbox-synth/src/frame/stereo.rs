use crate::frame::Frame;

pub type StereoFrame<T> = Frame<T, 2>;

impl<T> StereoFrame<T> {
    pub const LEFT_IDX: usize = 0;
    pub const RIGHT_IDX: usize = 1;

    pub fn left(&self) -> &T {
        &self[Self::LEFT_IDX]
    }

    pub fn left_mut(&mut self) -> &mut T {
        &mut self[Self::LEFT_IDX]
    }

    pub fn right(&self) -> &T {
        &self[Self::RIGHT_IDX]
    }

    pub fn right_mut(&mut self) -> &mut T {
        &mut self[Self::RIGHT_IDX]
    }
}
