#![allow(dead_code)]

use petgraph::graph::{DiGraph, NodeIndex};
use slotmap::SlotMap;

use crate::{
    block::Block,
    frame::{MonoFrame, StereoFrame},
};

slotmap::new_key_type! {
    pub struct SynthNodeId;
    pub struct SynthInputPortId;
    pub struct SynthOutputPortId;
    pub struct SynthBufferId;
}

#[derive(Default)]
struct SynthPool<const FRAME_COUNT: usize> {
    buffers: SlotMap<SynthBufferId, SynthData<FRAME_COUNT>>,
    free_mono_audio: Vec<SynthBufferId>,
    free_stereo_audio: Vec<SynthBufferId>,
    free_control: Vec<SynthBufferId>,
    free_integer: Vec<SynthBufferId>,
    free_trigger: Vec<SynthBufferId>,
}

/// Construction.
impl<const FRAME_COUNT: usize> SynthPool<FRAME_COUNT> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(buffers: usize) -> Self {
        Self {
            buffers: SlotMap::with_capacity_and_key(buffers),
            ..Default::default()
        }
    }
}

/// Buffer management.
impl<const FRAME_COUNT: usize> SynthPool<FRAME_COUNT> {
    // TODO: maybe a better API? Right now anyone with write access can also allocate...
    pub fn allocate_buffer(&mut self, ty: SynthPortType) -> SynthBufferId {
        match ty {
            SynthPortType::MonoAudio => self
                .free_mono_audio
                .pop()
                .unwrap_or_else(|| self.buffers.insert(SynthData::MonoAudio(Block::default()))),
            SynthPortType::StereoAudio => self.free_stereo_audio.pop().unwrap_or_else(|| {
                self.buffers
                    .insert(SynthData::StereoAudio(Block::default()))
            }),
            SynthPortType::Control => self
                .free_control
                .pop()
                .unwrap_or_else(|| self.buffers.insert(SynthData::Control(Block::default()))),
            SynthPortType::Integer => self
                .free_integer
                .pop()
                .unwrap_or_else(|| self.buffers.insert(SynthData::Integer(Block::default()))),
            SynthPortType::Trigger => self
                .free_trigger
                .pop()
                .unwrap_or_else(|| self.buffers.insert(SynthData::Trigger(Block::default()))),
        }
    }

    pub fn free_buffer(&mut self, buffer_id: SynthBufferId) {
        match self.buffers[buffer_id] {
            SynthData::MonoAudio(_) => {
                self.free_mono_audio.push(buffer_id);
            }
            SynthData::StereoAudio(_) => {
                self.free_stereo_audio.push(buffer_id);
            }
            SynthData::Control(_) => {
                self.free_control.push(buffer_id);
            }
            SynthData::Integer(_) => {
                self.free_integer.push(buffer_id);
            }
            SynthData::Trigger(_) => {
                self.free_trigger.push(buffer_id);
            }
        }
    }
}

type DynSynthOpFn<const FRAME_COUNT: usize> =
    dyn Fn(&mut SynthPool<FRAME_COUNT>, &[SynthBufferId], &mut [SynthBufferId]);

type DynAllocOpFn<const FRAME_COUNT: usize> =
    dyn Fn(&mut SynthPool<FRAME_COUNT>, &[SynthPortType]) -> Vec<SynthBufferId>;

struct SynthOp<'op, const FRAME_COUNT: usize> {
    synth_op_fn: &'op DynSynthOpFn<FRAME_COUNT>,
    output_buffers: Vec<SynthBufferId>,
}

type InnerGraph = DiGraph<SynthNodeId, ()>;

struct Synth<const FRAME_COUNT: usize> {
    di_graph: InnerGraph,
    nodes: SlotMap<SynthNodeId, SynthNode<FRAME_COUNT>>,
}

struct SynthNode<const FRAME_COUNT: usize> {
    name: String,
    alloc_fn: Box<DynAllocOpFn<FRAME_COUNT>>,
    synth_fn: Box<DynSynthOpFn<FRAME_COUNT>>,
}

impl<const FRAME_COUNT: usize> SynthNode<FRAME_COUNT> {
    pub fn new(
        name: impl Into<String>,
        alloc_fn: impl Fn(&mut SynthPool<FRAME_COUNT>, &[SynthPortType]) -> Vec<SynthBufferId> + 'static,
        synth_fn: impl Fn(&mut SynthPool<FRAME_COUNT>, &[SynthBufferId], &mut [SynthBufferId]) + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            alloc_fn: Box::new(alloc_fn),
            synth_fn: Box::new(synth_fn),
        }
    }
}

enum SynthData<const FRAME_COUNT: usize> {
    /// Audio samples in range `-1.0..=1.0`.
    MonoAudio(Block<MonoFrame<f32>, FRAME_COUNT>),
    /// Audio samples in range `-1.0..=1.0`, interlaced.
    StereoAudio(Block<StereoFrame<f32>, FRAME_COUNT>),
    /// Arbitrary floating-point data.
    Control(Block<f32, FRAME_COUNT>),
    /// Arbitrary integer data.
    Integer(Block<i32, FRAME_COUNT>),
    /// Per-sample event triggers.
    Trigger(Block<bool, FRAME_COUNT>),
}

enum SynthPortType {
    MonoAudio,
    StereoAudio,
    Control,
    Integer,
    Trigger,
}

struct SynthInputPort {
    tys: Vec<SynthPortType>,
}

struct SynthOutputPort {
    ty: SynthPortType,
}

/// Construction.
impl<const FRAME_COUNT: usize> Synth<FRAME_COUNT> {
    pub fn new() -> Self {
        Self {
            di_graph: DiGraph::new(),
            nodes: SlotMap::with_key(),
        }
    }

    pub fn with_capacity(nodes: usize) -> Self {
        Self {
            di_graph: DiGraph::with_capacity(nodes, 0),
            nodes: SlotMap::with_capacity_and_key(nodes),
        }
    }
}

struct SynthPipeline<'op, const FRAME_COUNT: usize> {
    synth_ops: Vec<SynthOp<'op, FRAME_COUNT>>,
}

impl<const FRAME_COUNT: usize> SynthPipeline<'_, FRAME_COUNT> {
    pub fn render<'pipeline>(
        &'pipeline mut self,
        pool: &'pipeline mut SynthPool<FRAME_COUNT>,
    ) -> &'pipeline SynthData<FRAME_COUNT> {
        for op in &mut self.synth_ops {
            (op.synth_op_fn)(pool, &[], &mut op.output_buffers);
        }
        // TODO: this is obviously not what we want. We need a way to specify the output buffer (special node?).
        // return last op last output buffer
        &pool.buffers[*self
            .synth_ops
            .last()
            .unwrap()
            .output_buffers
            .last()
            .unwrap()]
    }
}

/// Execution.
impl<const FRAME_COUNT: usize> Synth<FRAME_COUNT> {
    pub fn pipeline<'synth>(
        &'synth mut self,
        pool: &mut SynthPool<FRAME_COUNT>,
    ) -> SynthPipeline<'synth, FRAME_COUNT> {
        let ops = petgraph::algo::toposort(&self.di_graph, None)
            // Map on result! Toposort can fail.
            .map(|v| {
                v.into_iter().map(|ix| {
                    // This shouldn't fail as long as the algo doesn't output invalid indices.
                    let id = *self.di_graph.node_weight(ix).expect("toposort failed");
                    let node = &self.nodes[id];
                    let output_buffers = (node.alloc_fn)(pool, &[]);
                    SynthOp {
                        synth_op_fn: &node.synth_fn,
                        output_buffers,
                    }
                })
            })
            // TODO: convert cycle to user friendly error, bubble up
            .expect("cycle")
            .collect();
        SynthPipeline { synth_ops: ops }
    }
}

/// Node management.
impl<const FRAME_COUNT: usize> Synth<FRAME_COUNT> {
    fn find_node_ix(&self, node_id: SynthNodeId) -> Option<NodeIndex> {
        self.di_graph
            .node_indices()
            .find(|ix| self.di_graph[*ix] == node_id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &SynthNode<FRAME_COUNT>> + use<'_, FRAME_COUNT> {
        self.nodes.values()
    }

    pub fn node_ids(&self) -> impl Iterator<Item = SynthNodeId> + use<'_, FRAME_COUNT> {
        self.di_graph.node_indices().map(|ix| self.di_graph[ix])
    }

    pub fn node(&self, node_id: SynthNodeId) -> Option<&SynthNode<FRAME_COUNT>> {
        self.nodes.get(node_id)
    }

    pub fn add_node(&mut self, node: SynthNode<FRAME_COUNT>) -> SynthNodeId {
        let node_id = self.nodes.insert(node);
        self.di_graph.add_node(node_id);
        node_id
    }

    pub fn remove_node(&mut self, node_id: SynthNodeId) -> Option<SynthNode<FRAME_COUNT>> {
        let node_ix = self.find_node_ix(node_id)?;
        self.di_graph.remove_node(node_ix);
        self.nodes.remove(node_id)
    }
}

#[cfg(test)]
#[test]
fn test() {
    use cpal::traits::{DeviceTrait as _, HostTrait as _, StreamTrait as _};
    use rb::{RbConsumer as _, RbProducer as _, RB as _};

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();
    println!("default output config: {:#?}", config);
    let rb = rb::SpscRb::new(1024);
    let consumer = rb.consumer();
    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let result = consumer.read(data);
                match result {
                    Ok(n) => {
                        if n < data.len() {
                            eprintln!("partial underflow");
                        }
                    }
                    Err(rb::RbError::Empty) => {
                        eprintln!("total underflow")
                    }
                    Err(err) => panic!("unexpected error: {}", err),
                }
            },
            |err| eprintln!("an error occurred on the output stream: {}", err),
            None,
        )
        .unwrap();

    let mut synth: Synth<128> = Synth::new();

    // this is pretty slick...
    let _synth_id = synth.add_node(SynthNode::new(
        "random noise in left channel",
        |pool, _inputs| vec![pool.allocate_buffer(SynthPortType::StereoAudio)],
        |pool, _, outputs| {
            let buffer = &mut pool.buffers[outputs[0]];
            if let SynthData::StereoAudio(block) = buffer {
                for frame in block.as_frames_mut() {
                    *frame.left_mut() = rand::random();
                }
            }
        },
    ));

    let mut pool = SynthPool::new();
    let mut pipeline = synth.pipeline(&mut pool);
    let producer = rb.producer();
    stream.play().unwrap();

    loop {
        let data = pipeline.render(&mut pool);
        if let SynthData::StereoAudio(frames) = data {
            producer.write_blocking(frames.as_samples());
        }
    }
}
