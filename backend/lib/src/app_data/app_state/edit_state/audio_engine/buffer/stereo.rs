use rb::RB;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct StereoFrame {
    pub left: f64,
    pub right: f64,
}

impl StereoFrame {
    pub const CHANNEL_COUNT: u16 = 2;
}

pub struct StereoBuffer {
    pub inner: rb::SpscRb<StereoFrame>,
    /// Relative to the start of the frame.
    pub sample_index: usize,
}

impl StereoBuffer {
    pub fn producer(&mut self) -> rb::Producer<StereoFrame> {
        self.inner.producer()
    }

    pub fn consumer(&mut self) -> rb::Consumer<StereoFrame> {
        self.inner.consumer()
    }
}
