use crate::frame::Frame;

pub type MonoFrame<T> = Frame<T, 1>;

impl<T> MonoFrame<T> {
    pub const VALUE_IDX: usize = 0;

    pub fn value(&self) -> &T {
        &self[Self::VALUE_IDX]
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self[Self::VALUE_IDX]
    }
}

impl<T> AsRef<T> for MonoFrame<T> {
    fn as_ref(&self) -> &T {
        self.value()
    }
}
