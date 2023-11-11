use gen_value::vec::GenVec;
use std::cell::RefCell;

type StreamIdxProd = usize;
pub type StreamIdx = (StreamIdxProd, StreamIdxProd);
type StreamsGenVec = GenVec<cpal::Stream, StreamIdxProd, StreamIdxProd>;

impl From<&StreamHandle> for StreamIdx {
    fn from(x: &StreamHandle) -> Self {
        x.idx
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct StreamHandle {
    idx: StreamIdx,
}

#[derive(Default)]
pub struct Streams {
    inner: StreamsGenVec,
}

impl Streams {
    pub fn from_gen_vec(streams: StreamsGenVec) -> Self {
        Self { inner: streams }
    }

    pub fn insert(
        &mut self,
        stream: cpal::Stream,
    ) -> Result<StreamHandle, gen_value::Error> {
        self.inner
            .insert(stream)
            .map(|x| StreamHandle { idx: x })
    }

    pub fn get(
        &self,
        idx: impl Into<StreamIdx>,
    ) -> Result<&cpal::Stream, gen_value::Error> {
        self.inner.get(idx.into())
    }

    fn remove(
        &mut self,
        idx: impl Into<StreamIdx>,
    ) -> Result<(), gen_value::Error> {
        self.inner.remove(idx.into())
    }
}

impl From<StreamsGenVec> for Streams {
    fn from(streams: StreamsGenVec) -> Self {
        Self::from_gen_vec(streams)
    }
}

impl Drop for StreamHandle {
    fn drop(&mut self) {
        STREAMS.with(|x| {
            x.borrow_mut()
                .remove(self.idx)
                .unwrap_or_else(|e| {
                    tracing::error!("unable to remove stream: {}", e)
                });
        });
    }
}

thread_local! {
    pub static STREAMS: RefCell<Streams> = RefCell::new(Default::default());
}
