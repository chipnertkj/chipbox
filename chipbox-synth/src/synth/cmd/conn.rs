pub use slot::SlotTy;
pub mod slot;

use crate::synth::{self, cmd};
use petgraph::csr::IndexType;

#[derive(Debug, thiserror::Error)]
#[error("expected {expected_tys:?}, found {found_ty:?}")]
pub struct ConnTyMismatch<Ix: IndexType> {
    pub conn: Conn<Ix>,
    pub found_ty: SlotTy,
    pub expected_tys: Vec<SlotTy>,
}

#[derive(Clone, Debug, derive_more::Display)]
#[display("{src_cmd}:{src_slot} -> {dest_cmd}:{dest_slot}")]
pub struct Conn<Ix: IndexType> {
    pub src_cmd: synth::CmdIx<Ix>,
    pub dest_cmd: synth::CmdIx<Ix>,
    pub src_slot: usize,
    pub dest_slot: usize,
}

impl<Ix: IndexType> Conn<Ix> {
    pub fn new(
        (src_cmd, src_slot): (synth::CmdIx<Ix>, usize),
        (dest_cmd, dest_slot): (synth::CmdIx<Ix>, usize),
    ) -> Self {
        Self {
            src_cmd,
            src_slot,
            dest_cmd,
            dest_slot,
        }
    }
}

impl<Ix: IndexType> Conn<Ix> {
    pub fn src_cmd<'a, C: cmd::GraphCmd>(
        &self,
        synth: &'a synth::Graph<C, Ix>,
    ) -> Option<&'a cmd::Cmd<C>> {
        synth.cmd(self.src_cmd)
    }

    pub fn dest_cmd<'a, C: cmd::GraphCmd>(
        &self,
        synth: &'a synth::Graph<C, Ix>,
    ) -> Option<&'a cmd::Cmd<C>> {
        synth.cmd(self.dest_cmd)
    }
}
