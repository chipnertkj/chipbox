use super::data::{DataType, PortLabel};

slotmap::new_key_type! {
    pub struct NodeId;
}

pub struct Node {
    pub cmd: Cmd,
    pub name: String,
}

impl Node {
    pub fn new(name: impl Into<String>, cmd: impl Into<Cmd>) -> Self {
        Self {
            name: name.into(),
            cmd: cmd.into(),
        }
    }
}

pub trait GraphCmd {
    fn output_label(&self, port_ix: usize) -> Option<PortLabel>;
}

pub enum Cmd {
    SynthInputCmd(SynthInputCmd),
}

pub struct SynthInputCmd {
    out_labels: (),
}

impl SynthInputCmd {
    const PITCH_OUT_IX: usize = 0;

    pub const fn pitch_out_ix() -> usize {
        Self::PITCH_OUT_IX
    }
}

impl GraphCmd for SynthInputCmd {
    fn output_label(&self, port_ix: usize) -> Option<PortLabel> {
        match port_ix {
            Self::PITCH_OUT_IX => Some(PortLabel::new("pitch", DataType::Signal)),
            _ => None,
        }
    }
}
