use rb::RB;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct MonoFrame {
    pub center: f64,
}

impl MonoFrame {
    pub const CHANNEL_COUNT: u16 = 1;
}

pub struct MonoBuffer {
    pub inner: rb::SpscRb<MonoFrame>,
    /// Relative to the start of the frame.
    pub sample_index: usize,
}

impl MonoBuffer {
    pub fn producer(&mut self) -> rb::Producer<MonoFrame> {
        self.inner.producer()
    }

    pub fn consumer(&mut self) -> rb::Consumer<MonoFrame> {
        self.inner.consumer()
    }
}
