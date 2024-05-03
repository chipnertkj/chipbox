use gen_value::vec::GenVec;
pub use thread_local::{
    clear_streams, init_streams, with_streams, with_streams_mut,
};

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

mod thread_local {
    impl Drop for super::StreamHandle {
        fn drop(&mut self) {
            let _ = STREAMS.try_with(|streams| {
                match &mut *streams.borrow_mut() {
                    Some(streams) => {
                        match streams.remove(self.idx) {
                            Ok(_) => {
                                tracing::info!("dropped stream. id: `{:?}`", self.idx);
                            }
                            Err(_) => {
                                tracing::warn!("stream already dropped. id: `{:?}`", self.idx);
                            }
                        }
                    }
                    None => {
                        tracing::warn!("stream handle dropped while STREAMS not initialized. id: `{:?}`", self.idx);
                    }
                };
            });
        }
    }

    use super::Streams;
    use std::cell::RefCell;
    use std::sync::Mutex;

    pub fn with_streams<F, T>(f: F) -> T
    where
        F: FnOnce(&Streams) -> T,
    {
        verify_thread_id();
        STREAMS.with(|x| match &*x.borrow() {
            Some(x) => f(x),
            None => panic!(
                "STREAMS thread-local accessed but not initialized on thread {:?}",
                std::thread::current().id()
            ),
        })
    }

    pub fn with_streams_mut<F, T>(f: F) -> T
    where
        F: FnOnce(&mut Streams) -> T,
    {
        verify_thread_id();
        STREAMS.with(|x| match &mut *x.borrow_mut() {
            Some(x) => f(x),
            None => panic!(
                "STREAMS thread-local accessed but not initialized on thread {:?}",
                std::thread::current().id()
            ),
        })
    }

    fn verify_thread_id() {
        let current_thread_id = std::thread::current().id();
        let stream_thread_id = STREAM_THREAD_ID
            .lock()
            .unwrap();
        match &*stream_thread_id {
            Some(stream_thread_id) => assert_eq!(
                current_thread_id,
                *stream_thread_id,
                "STREAMS thread-local initialized on thread {stream_thread_id:?}, \
                but accessed from thread {current_thread_id:?}",
            ),
            None => panic!(
                "STREAMS thread-local not initialized on thread {:?}",
                std::thread::current().id()
            ),
        }
    }

    pub fn init_streams() {
        tracing::info!("initializing thread-local storage for streams");
        STREAMS.with_borrow_mut(|streams_opt| {
            let mut stream_thread_id = STREAM_THREAD_ID
                .lock()
                .unwrap();
            match &*stream_thread_id
            {
                Some(thread_id) => {
                    panic!(
                        "STREAMS thread-local already initialized on thread {thread_id:?}",
                    );
                }
                None => {
                    let current_thread_id = std::thread::current().id();
                    *stream_thread_id = Some(current_thread_id);
                    *streams_opt = Some(Default::default());
                    tracing::info!(
                        "initialized STREAMS thread-local on thread {current_thread_id:?}",
                    );
                }
            }
        });
    }

    pub fn clear_streams() {
        tracing::info!("clearing thread-local storage for streams");
        let stream_count = STREAMS.with(|x| {
            x.borrow()
                .as_ref()
                .map(|x| x.inner.len())
                .unwrap_or_default()
        });
        STREAMS.with_borrow_mut(|streams_opt| {
            *streams_opt = None;
        });
        tracing::info!("removed streams: {stream_count}",);
    }

    static STREAM_THREAD_ID: Mutex<Option<std::thread::ThreadId>> =
        Mutex::new(None);

    thread_local! {
        static STREAMS: RefCell<Option<Streams>> = const { RefCell::new(None) };
    }
}
