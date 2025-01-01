#[derive(
    Clone,
    Copy,
    derive_more::Debug,
    derive_more::From,
    derive_more::Into,
    derive_more::AsRef,
    derive_more::AsMut,
)]
#[debug("timestamp/{}", self.sample_n)]
#[repr(transparent)]
pub struct Timestamp {
    pub sample_n: u64,
}

impl Timestamp {
    pub fn to_duration(self, sample_rate: f64) -> std::time::Duration {
        std::time::Duration::from_secs_f64(self.sample_n as f64 / sample_rate)
    }

    pub fn display(self, sample_rate: f64) -> String {
        let dur = self.to_duration(sample_rate);
        format!("{:.3}s", dur.as_secs_f64())
    }
}
