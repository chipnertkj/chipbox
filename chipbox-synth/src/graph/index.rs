use super::{
    cmd::{Conn, GraphCmd},
    CmdGraph,
};
use petgraph::graph::{EdgeIndex, IndexType, NodeIndex};

/// An identifier for a [command](cmd::Cmd) in a [`Graph`].
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display)]
#[display("node/{:?}", self.0.index())]
pub struct CmdIx<Ix>(pub(super) NodeIndex<Ix>)
where
    Ix: IndexType;

impl<Ix> CmdIx<Ix>
where
    Ix: IndexType,
{
    /// Get access to the [`Cmd`](cmd::Cmd) for this identifier from the [`Graph`].
    pub fn cmd<Cmd>(self, graph: &CmdGraph<Cmd, Ix>) -> Option<&Cmd>
    where
        Cmd: GraphCmd,
    {
        graph.cmd(self)
    }

    /// Get mutable access to the [`Cmd`](cmd::Cmd) for this identifier from the [`Graph`].
    pub fn cmd_mut<Cmd>(self, graph: &mut CmdGraph<Cmd, Ix>) -> Option<&mut Cmd>
    where
        Cmd: GraphCmd,
    {
        graph.cmd_mut(self)
    }
}

/// An identifier for a [connection](cmd::Conn) between two [commands](cmd::Cmd) in a [`Graph`].
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display)]
#[display("edge/{:?}", self.0.index())]
pub struct ConnIx<Ix>(pub(super) EdgeIndex<Ix>)
where
    Ix: IndexType;

impl<Ix> ConnIx<Ix>
where
    Ix: IndexType,
{
    /// Get access to the [`Conn`](cmd::Conn) for this identifier from the [`Graph`].
    pub fn conn<Cmd>(self, graph: &CmdGraph<Cmd, Ix>) -> Option<&Conn>
    where
        Cmd: GraphCmd,
    {
        graph.conn(self)
    }

    /// Get mutable access to the [`Conn`](cmd::Conn) for this identifier from the [`Graph`].
    pub fn conn_mut<Cmd>(self, graph: &mut CmdGraph<Cmd, Ix>) -> Option<&mut Conn>
    where
        Cmd: GraphCmd,
    {
        graph.conn_mut(self)
    }
}
