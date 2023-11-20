pub use self::mono::{MonoBuffer, MonoFrame};
pub use self::stereo::{StereoBuffer, StereoFrame};

mod mono;
mod stereo;

pub enum Buffer {
    Mono(MonoBuffer),
    Stereo(StereoBuffer),
}

pub enum Producer {
    Mono(rb::Producer<MonoFrame>),
    Stereo(rb::Producer<StereoFrame>),
}

pub enum Consumer {
    Mono(rb::Consumer<MonoFrame>),
    Stereo(rb::Consumer<StereoFrame>),
}

pub enum BufferConfig {
    Mono { length: usize },
    Stereo { length: usize },
}

impl Buffer {
    pub const SUPPORTED_CHANNEL_COUNTS: [u16; 2] =
        [MonoFrame::CHANNEL_COUNT, StereoFrame::CHANNEL_COUNT];

    pub fn from_config(config: &BufferConfig) -> Self {
        match config {
            BufferConfig::Mono { length } => Self::Mono(MonoBuffer {
                inner: rb::SpscRb::new(*length),
                sample_index: 0,
            }),
            BufferConfig::Stereo { length } => Self::Stereo(StereoBuffer {
                inner: rb::SpscRb::new(*length),
                sample_index: 0,
            }),
        }
    }

    pub fn producer(&mut self) -> Producer {
        match self {
            Self::Mono(buffer) => Producer::Mono(buffer.producer()),
            Self::Stereo(buffer) => Producer::Stereo(buffer.producer()),
        }
    }

    pub fn consumer(&mut self) -> Consumer {
        match self {
            Self::Mono(buffer) => Consumer::Mono(buffer.consumer()),
            Self::Stereo(buffer) => Consumer::Stereo(buffer.consumer()),
        }
    }
}
