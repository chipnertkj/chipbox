use crate::{
    block::Block,
    frame::{MonoFrame, StereoFrame},
    graph::{CmdData, CmdDataDesc, CmdGraph, GraphCmd, InputSlot, OutputSlot},
    timestamp::Timestamp,
};
use enum_as_inner::EnumAsInner;
use petgraph::graph::IndexType;

#[derive(Debug)]
pub struct InputCmd {
    output: OutputSlot<Desc>,
}

impl InputCmd {
    pub fn new() -> Self {
        Self {
            output: OutputSlot::new("time", Desc::Timestamp),
        }
    }
}

impl Default for InputCmd {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphCmd for InputCmd {
    type Data = Data;
    type Desc = Desc;
    fn input_slots(&self) -> &[InputSlot<Desc>] {
        &[]
    }
    fn output_slots(&self) -> &[OutputSlot<Desc>] {
        std::slice::from_ref(&self.output)
    }
    fn render(&self, _inputs: &[Option<Self::Data>]) -> Self::Data {
        todo!()
    }
}

/// The output node of a synthesizer.
///
/// Contains only one input node, which is used to feed data to the output buffer.
/// Accepts [`Desc::MonoSamples`] and [`Desc::StereoSamples`].
#[derive(Debug)]
pub struct OutputCmd {
    input: InputSlot<Desc>,
}

impl OutputCmd {
    /// Create a new output node.
    pub fn new() -> Self {
        Self {
            input: InputSlot::new("graph output", [Desc::MonoSamples, Desc::StereoSamples]),
        }
    }
}

impl Default for OutputCmd {
    /// Create a new output node.
    /// Equivalent to [`Self::new()`].
    fn default() -> Self {
        Self::new()
    }
}

impl GraphCmd for OutputCmd {
    type Data = Data;
    type Desc = Desc;
    fn input_slots(&self) -> &[InputSlot<Desc>] {
        std::slice::from_ref(&self.input)
    }
    fn output_slots(&self) -> &[OutputSlot<Desc>] {
        &[]
    }
    fn render(&self, inputs: &[Option<Self::Data>]) -> Self::Data {
        todo!()
    }
}

/// Primary waveform generator input data.
#[derive(Debug, Default)]
pub enum WaveformDesc {
    #[default]
    Preset,
}

/// Waveform generator node.
///
/// Generates a waveform based on the set parameters.
/// Pitch and time may be modulated
#[derive(Debug)]
pub struct WaveformCmd {
    inputs: [InputSlot<Desc>; 2],
    output: OutputSlot<Desc>,
    pub waveform: WaveformDesc,
}

impl WaveformCmd {
    pub fn new(waveform: WaveformDesc) -> Self {
        Self {
            inputs: [
                InputSlot::new("pitch", [Desc::MonoSamples]),
                InputSlot::new("time", [Desc::Timestamp]),
            ],
            output: OutputSlot::new("waveform", Desc::MonoSamples),
            waveform,
        }
    }
}

impl Default for WaveformCmd {
    fn default() -> Self {
        Self::new(WaveformDesc::default())
    }
}

impl GraphCmd for WaveformCmd {
    type Data = Data;
    type Desc = Desc;
    fn input_slots(&self) -> &[InputSlot<Desc>] {
        &self.inputs
    }
    fn output_slots(&self) -> &[OutputSlot<Desc>] {
        std::slice::from_ref(&self.output)
    }
    fn render(&self, inputs: &[Option<Self::Data>]) -> Self::Data {
        todo!()
    }
}

#[enum_delegate::implement(GraphCmd)]
#[derive(EnumAsInner, Debug)]
pub enum Cmd {
    /// Synthesizer input.
    Input(InputCmd),
    /// Synthesizer output.
    Output(OutputCmd),
    /// Waveform generator.
    Waveform(WaveformCmd),
}

#[derive(Debug, Clone, Copy, PartialEq, derive_more::Display)]
pub enum Desc {
    /// Corresponds to [`Data::MonoSamples`].
    #[display("samples/1ch")]
    MonoSamples,
    /// Corresponds to [`Data::StereoSamples`].
    #[display("samples/2ch")]
    StereoSamples,
    /// Corresponds to [`Data::Timestamp`].
    #[display("time")]
    Timestamp,
}

impl CmdDataDesc for Desc {}

// TODO: use preallocation in a central cache with generational indices instead to reduce payload size
#[derive(Debug)]
pub enum Data {
    MonoSamples(Block<MonoFrame<f64>, 256>),
    StereoSamples(Block<StereoFrame<f64>, 256>),
    /// Time period between the value of the [`Timestamp`] and the same value plus the `FRAME_COUNT` generic parameter.
    Timestamp(Timestamp),
}

impl CmdData for Data {}

pub struct Synth<Ix>
where
    Ix: IndexType,
{
    pub graph: CmdGraph<Cmd, Ix>,
}

impl<Ix> Synth<Ix>
where
    Ix: IndexType,
{
    /// Create an empty synth with pre-allocated space for [commands](CmdGraph) and [connections](crate::cmd::Conn).
    pub fn with_capacity(cmds: usize, conns: usize) -> Self {
        Self {
            graph: CmdGraph::with_capacity(cmds, conns),
        }
    }

    pub fn render_mono(&self, block: &mut MonoFrame<f64>, sample_rate_hz: f64) -> Data {
        todo!()
    }

    pub fn render_stereo(&self, block: &mut StereoFrame<f64>, sample_rate_hz: f64) -> Data {
        todo!()
    }
}
