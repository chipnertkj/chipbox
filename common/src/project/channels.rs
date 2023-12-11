pub mod synth;

pub use self::synth::SynthChannel;

use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    derive_more::Display,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    Hash,
)]
pub enum ChannelIdx {
    Synth(usize),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Channels {
    synth: Vec<SynthChannel>,
    /// Order in which channels are displayed.
    /// Implementation ensures that no duplicate channel ids are used.
    order: Vec<ChannelIdx>,
}

impl Channels {
    pub fn synth(&self) -> &[SynthChannel] {
        &self.synth
    }

    pub fn synth_mut(&mut self) -> &mut [SynthChannel] {
        &mut self.synth
    }

    pub fn order(&self) -> &[ChannelIdx] {
        &self.order
    }

    /// Find the *order vector* index of the given `ChannelIdx`.
    ///
    /// Returns an error if the given item is not in the order vector.
    fn find_order_idx_pos(&self, idx: ChannelIdx) -> Result<usize, String> {
        self.order
            .iter()
            .position(|i| *i == idx)
            .ok_or_else(|| format!("unable to find index {idx} in order vec"))
    }

    /// Swap the order of channels `A` and `B`.
    pub fn swap_idx_order(
        &mut self,
        idx_a: ChannelIdx,
        idx_b: ChannelIdx,
    ) -> Result<(), String> {
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
    ) -> Result<(), String> {
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
    ) -> Result<(), String> {
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
    pub fn remove_channel(&mut self, idx: ChannelIdx) -> Result<(), String> {
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
mod tests {
    use super::*;

    #[test]
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
        // Channel 0 is now at index 2.
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
        // Channel 0 is now at index 1.
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
        // Channel 0 is now at index 2.
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
