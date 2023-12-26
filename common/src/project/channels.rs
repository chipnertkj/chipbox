pub mod synth;

pub use self::synth::SynthChannel;

#[derive(Debug, PartialEq, Eq)]
///
pub struct OrderFromChannelIdxError {
    idx: ChannelIdx,
}

impl std::error::Error for OrderFromChannelIdxError {}

impl std::fmt::Display for OrderFromChannelIdxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "channel index not found in order vector: {:?}", self.idx)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
/// Channel identifier.
/// Corresponds to the item index in the internal vector where the channel is stored.
///
/// Does not represent the order in which channels should be displayed.
pub enum ChannelIdx {
    Synth(usize),
    // Add automation, audio, event channels...
}

#[derive(
    Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq,
)]
/// Audio channels of a project.
///
/// The order in which channels are displayed is stored in the `order` vector.
pub struct Channels {
    synth: Vec<SynthChannel>,
    /// Order in which channels are displayed.
    /// Implementation ensures that no duplicate channel ids are used.
    order: Vec<ChannelIdx>,
}

impl Channels {
    /// Get a slice of all the synth channels.
    pub fn synth(&self) -> &[SynthChannel] {
        &self.synth
    }

    /// Get a mutable slice of all the synth channels.
    pub fn synth_mut(&mut self) -> &mut [SynthChannel] {
        &mut self.synth
    }

    /// Get a slice showing the order in which channels should be displayed.
    pub fn order(&self) -> &[ChannelIdx] {
        &self.order
    }

    /// Find the *order vector index* of the given `ChannelIdx`.
    ///
    /// Returns an error if the given item is not in the order vector.
    fn find_order_idx_pos(
        &self,
        idx: ChannelIdx,
    ) -> Result<usize, OrderFromChannelIdxError> {
        self.order
            .iter()
            .position(|i| *i == idx)
            .ok_or(OrderFromChannelIdxError { idx })
    }

    /// Swap the order of channels `A` and `B`.
    pub fn swap_idx_order(
        &mut self,
        idx_a: ChannelIdx,
        idx_b: ChannelIdx,
    ) -> Result<(), OrderFromChannelIdxError> {
        if idx_a == idx_b {
            Ok(())
        } else {
            let idx_a = self.find_order_idx_pos(idx_a)?;
            let idx_b = self.find_order_idx_pos(idx_b)?;
            self.order.swap(idx_a, idx_b);
            Ok(())
        }
    }

    /// Move order of channel `A` so that it appears before channel `B`.
    ///
    /// Returns an error if the channels are the same.
    pub fn move_idx_before(
        &mut self,
        idx_a: ChannelIdx,
        idx_b: ChannelIdx,
    ) -> Result<(), OrderFromChannelIdxError> {
        if idx_a == idx_b {
            Ok(())
        } else {
            self.order
                .remove(self.find_order_idx_pos(idx_a)?);
            self.order
                .insert(self.find_order_idx_pos(idx_b)?, idx_a);
            Ok(())
        }
    }

    /// Move order of channel `A` so that it appears after channel `B`.
    ///
    /// Returns an error if the channels are the same.
    pub fn move_idx_after(
        &mut self,
        idx_a: ChannelIdx,
        idx_b: ChannelIdx,
    ) -> Result<(), OrderFromChannelIdxError> {
        if idx_a == idx_b {
            Ok(())
        } else {
            self.order
                .remove(self.find_order_idx_pos(idx_a)?);
            self.order
                .insert(self.find_order_idx_pos(idx_b)? + 1, idx_a);
            Ok(())
        }
    }

    /// Remove the given channel.
    pub fn remove_channel(
        &mut self,
        idx: ChannelIdx,
    ) -> Result<(), OrderFromChannelIdxError> {
        // Remove from order vec.
        self.order
            .remove(self.find_order_idx_pos(idx)?);
        // Remove from internal channels vec.
        match idx {
            ChannelIdx::Synth(idx) => self.synth.remove(idx),
        };
        Ok(())
    }
}

#[cfg(test)]
/// Assert the behavior of `Channels` impl.
mod tests {
    use super::*;

    #[test]
    /// Assert the behavior of `find_order_idx_pos`.
    fn find_order_idx_pos() {
        let channels = Channels {
            order: vec![
                ChannelIdx::Synth(2),
                ChannelIdx::Synth(1),
                ChannelIdx::Synth(0),
            ],
            ..Default::default()
        };
        assert_eq!(channels.find_order_idx_pos(ChannelIdx::Synth(0)), Ok(2));
        assert_eq!(channels.find_order_idx_pos(ChannelIdx::Synth(1)), Ok(1));
        assert_eq!(channels.find_order_idx_pos(ChannelIdx::Synth(2)), Ok(0));
        assert!(channels
            .find_order_idx_pos(ChannelIdx::Synth(3))
            .is_err());
    }

    #[test]
    /// Assert the behavior of `swap_idx_order`.
    fn swap_idx_order() {
        let mut channels = Channels {
            order: vec![
                ChannelIdx::Synth(0),
                ChannelIdx::Synth(1),
                ChannelIdx::Synth(2),
            ],
            // Actual channels are not used in this test.
            ..Default::default()
        };
        // Swap channels 0 and 2.
        channels
            .swap_idx_order(ChannelIdx::Synth(0), ChannelIdx::Synth(2))
            .unwrap();
        // Channel 0 is now at order index 2.
        assert_eq!(
            channels.order,
            vec![
                ChannelIdx::Synth(2),
                ChannelIdx::Synth(1),
                ChannelIdx::Synth(0),
            ]
        );
    }

    #[test]
    /// Assert the behavior of `move_idx_before`.
    fn move_idx_before() {
        let mut channels = Channels {
            order: vec![
                ChannelIdx::Synth(0),
                ChannelIdx::Synth(1),
                ChannelIdx::Synth(2),
            ],
            // Actual channels are not used in this test.
            ..Default::default()
        };
        // Move channel 0 before channel 2.
        channels
            .move_idx_before(ChannelIdx::Synth(0), ChannelIdx::Synth(2))
            .unwrap();
        // Channel 0 is now at order index 1.
        assert_eq!(
            channels.order,
            vec![
                ChannelIdx::Synth(1),
                ChannelIdx::Synth(0),
                ChannelIdx::Synth(2),
            ]
        );
    }

    #[test]
    /// Assert the behavior of `move_idx_after`.
    fn move_idx_after() {
        let mut channels = Channels {
            order: vec![
                ChannelIdx::Synth(0),
                ChannelIdx::Synth(1),
                ChannelIdx::Synth(2),
            ],
            // Actual channels are not used in this test.
            ..Default::default()
        };
        // Move channel 0 after channel 2.
        channels
            .move_idx_after(ChannelIdx::Synth(0), ChannelIdx::Synth(2))
            .unwrap();
        // Channel 0 is now at order index 2.
        assert_eq!(
            channels.order,
            vec![
                ChannelIdx::Synth(1),
                ChannelIdx::Synth(2),
                ChannelIdx::Synth(0),
            ]
        );
    }
}
