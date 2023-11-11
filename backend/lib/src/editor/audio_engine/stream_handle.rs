use gen_value::vec::GenVec;
pub use thread_local::{with_streams, with_streams_mut};

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
        let handle = self
            .inner
            .insert(stream)
            .map(|x| StreamHandle { idx: x });
        tracing::info!("adding stream to thread-local storage: `{:?}`", handle);
        handle
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
        tracing::info!("dropping stream handle: `{:?}`", self);
        with_streams_mut(|streams| {
            streams
                .remove(self.idx)
                .unwrap_or_else(|e| {
                    tracing::error!("unable to remove stream: {}", e)
                });
            tracing::info!(
                "stream removed from thread-local storage: `{:?}`",
                self
            );
        });
    }
}

mod thread_local {
    use super::Streams;
    use std::cell::{Ref, RefCell};
    use std::rc::Rc;

    pub fn with_streams<T, F>(f: F) -> T
    where
        F: FnOnce(Ref<Streams>) -> T,
    {
        STREAMS.with(|x| f(x.clone().borrow()))
    }

    pub fn with_streams_mut<T, F>(f: F) -> T
    where
        F: FnOnce(&mut Streams) -> T,
    {
        STREAMS.with(|x| f(&mut x.borrow_mut()))
    }

    thread_local! {
        static STREAMS: Rc<RefCell<Streams>> = Rc::new(RefCell::new(Default::default()));
    }
}
