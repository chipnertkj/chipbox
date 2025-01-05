use crate::frame::Frame;

pub type MonoFrame<SampleT> = Frame<SampleT, 1>;

impl<SampleT> MonoFrame<SampleT> {
    pub const VALUE_IDX: usize = 0;

    pub fn value(&self) -> &SampleT {
        &self[Self::VALUE_IDX]
    }

    pub fn value_mut(&mut self) -> &mut SampleT {
        &mut self[Self::VALUE_IDX]
    }
}

impl<SampleT> AsRef<SampleT> for MonoFrame<SampleT> {
    fn as_ref(&self) -> &SampleT {
        self.value()
    }
}

impl<SampleT> AsMut<SampleT> for MonoFrame<SampleT> {
    fn as_mut(&mut self) -> &mut SampleT {
        self.value_mut()
    }
}
