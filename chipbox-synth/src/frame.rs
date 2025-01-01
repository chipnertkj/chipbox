pub mod mono;
pub mod stereo;

pub use mono::MonoFrame;
pub use stereo::StereoFrame;

#[derive(
    Debug,
    Clone,
    Copy,
    derive_more::Display,
    derive_more::From,
    derive_more::Into,
    derive_more::IntoIterator,
    derive_more::AsRef,
    derive_more::AsMut,
    derive_more::Index,
    derive_more::IndexMut,
    derive_more::Mul,
    derive_more::MulAssign,
)]
#[repr(transparent)]
pub struct Frame<SampleT, const CHANNEL_COUNT: usize> {
    data: [SampleT; CHANNEL_COUNT],
}
