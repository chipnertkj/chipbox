#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputSlot {
    pub name: String,
    pub tys: Vec<SlotTy>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OutputSlot {
    pub name: String,
    pub ty: SlotTy,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, derive_more::Display)]
pub enum SlotTy {
    /// A signal type with the given amount of channels, or unbounded if `0`.
    /// - An unbounded signal input accepts any number of channels.
    /// - An unbounded signal output may emit any number of channels.
    /// - A bounded signal input accepts only the specified number of channels.
    /// - A bounded signal output may only emit the specified number of channels.
    ///
    /// The difference is that the amount of channels in a bounded signal can only
    /// change if the command is reconfigured, while the amount of channels in an
    /// unbounded signal may change at any time.
    #[display("signal({})", SlotTy::display_signal_binding(self).expect("signal"))]
    Signal { bind_ch_n: usize },
    #[display("timestamp")]
    Timestamp,
    #[display("duration")]
    Duration,
}

impl SlotTy {
    fn display_signal_binding(&self) -> Option<String> {
        match self {
            SlotTy::Signal { bind_ch_n: binding } => {
                let str = (*binding != 0)
                    .then_some(format!("{binding}ch"))
                    .unwrap_or("unbounded".to_string());
                Some(str)
            }
            _ => None,
        }
    }
}
