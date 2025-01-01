pub use slot::{InputSlot, OutputSlot};

mod slot;
use crate::graph::GraphCmd;
use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
#[error("expected {expected_ty_descs:?}, found {found_ty_desc:?}")]
pub struct ConnTyMismatch<Cmd>
where
    Cmd: GraphCmd,
    <Cmd as GraphCmd>::Desc: Debug,
{
    pub conn: Conn,
    pub found_ty_desc: <Cmd as GraphCmd>::Desc,
    pub expected_ty_descs: Vec<<Cmd as GraphCmd>::Desc>,
}

#[derive(Clone, Debug, derive_more::Display)]
#[display("{src_slot} -> {dest_slot}")]
pub struct Conn {
    pub src_slot: usize,
    pub dest_slot: usize,
}

impl From<(usize, usize)> for Conn {
    fn from((src_slot, dest_slot): (usize, usize)) -> Self {
        Self::new(src_slot, dest_slot)
    }
}

impl Conn {
    pub fn new(src_slot: usize, dest_slot: usize) -> Self {
        Self {
            src_slot,
            dest_slot,
        }
    }
}
