use crate::block::{Block, StereoBlock};
use slotmap::SlotMap;

/// Type of data accepted by a port.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Audio,
    Signal,
    Trigger,
}

pub struct PortLabel<'name> {
    name: &'name str,
    ty: DataType,
}

impl<'name> PortLabel<'name> {
    pub fn new(name: &'name str, ty: DataType) -> Self {
        Self { name, ty }
    }
}

slotmap::new_key_type! {
    pub struct AudioBlockId;
    pub struct SignalBlockId;
    pub struct TriggerBlockId;
}

pub struct Dataset<const FRAME_COUNT: usize> {
    audio_blocks: SlotMap<AudioBlockId, StereoBlock<f64, FRAME_COUNT>>,
    signal_blocks: SlotMap<SignalBlockId, Block<f64, FRAME_COUNT>>,
    trigger_blocks: SlotMap<TriggerBlockId, Block<bool, FRAME_COUNT>>,
}

/// Construction.
impl<const FRAME_COUNT: usize> Dataset<FRAME_COUNT> {
    pub fn new() -> Self {
        Self {
            audio_blocks: SlotMap::with_key(),
            signal_blocks: SlotMap::with_key(),
            trigger_blocks: SlotMap::with_key(),
        }
    }

    pub fn with_capacity(audio_blocks: usize, signal_blocks: usize, trigger_blocks: usize) -> Self {
        Self {
            audio_blocks: SlotMap::with_capacity_and_key(audio_blocks),
            signal_blocks: SlotMap::with_capacity_and_key(signal_blocks),
            trigger_blocks: SlotMap::with_capacity_and_key(trigger_blocks),
        }
    }
}

/// Allocation.
impl<const FRAME_COUNT: usize> Dataset<FRAME_COUNT> {
    pub fn allocate_audio_block(&mut self) -> AudioBlockId {
        self.audio_blocks.insert(StereoBlock::default())
    }

    pub fn allocate_signal_block(&mut self) -> SignalBlockId {
        self.signal_blocks.insert(Block::default())
    }

    pub fn allocate_trigger_block(&mut self) -> TriggerBlockId {
        self.trigger_blocks.insert(Block::default())
    }
}

/// Access.
impl<const FRAME_COUNT: usize> Dataset<FRAME_COUNT> {
    pub fn with_audio_block<F, Output>(&self, id: AudioBlockId, f: F) -> Option<Output>
    where
        F: FnOnce(&StereoBlock<f64, FRAME_COUNT>) -> Output,
    {
        self.audio_blocks.get(id).map(f)
    }

    pub fn update_audio_block<F, Output>(&mut self, id: AudioBlockId, f: F) -> Option<Output>
    where
        F: FnOnce(&mut StereoBlock<f64, FRAME_COUNT>) -> Output,
    {
        self.audio_blocks.get_mut(id).map(f)
    }

    pub fn with_signal_block<F, Output>(&self, id: SignalBlockId, f: F) -> Option<Output>
    where
        F: FnOnce(&Block<f64, FRAME_COUNT>) -> Output,
    {
        self.signal_blocks.get(id).map(f)
    }

    pub fn update_signal_block<F, Output>(&mut self, id: SignalBlockId, f: F) -> Option<Output>
    where
        F: FnOnce(&mut Block<f64, FRAME_COUNT>) -> Output,
    {
        self.signal_blocks.get_mut(id).map(f)
    }

    pub fn with_trigger_block<F, Output>(&self, id: TriggerBlockId, f: F) -> Option<Output>
    where
        F: FnOnce(&Block<bool, FRAME_COUNT>) -> Output,
    {
        self.trigger_blocks.get(id).map(f)
    }

    pub fn update_trigger_block<F, Output>(&mut self, id: TriggerBlockId, f: F) -> Option<Output>
    where
        F: FnOnce(&mut Block<bool, FRAME_COUNT>) -> Output,
    {
        self.trigger_blocks.get_mut(id).map(f)
    }
}
