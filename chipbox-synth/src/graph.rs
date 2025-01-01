pub use cmd::{CmdData, CmdDataDesc, Conn, ConnTyMismatch, GraphCmd, InputSlot, OutputSlot};
pub use index::{CmdIx, ConnIx};

mod cmd;
mod index;
use petgraph::{
    graph::IndexType,
    stable_graph::StableDiGraph,
    visit::{EdgeRef, IntoEdgeReferences},
};

/// The underlying graph type used by [`CmdGraph`].
type InnerGraph<Cmd, Ix> = StableDiGraph<Cmd, Conn, Ix>;

/// Errors that can occur when executing operations on a [`CmdGraph`].
#[derive(Debug, thiserror::Error)]
pub enum GraphError<Cmd, Ix>
where
    Cmd: GraphCmd,
    Ix: IndexType,
{
    /// Command for the supplied identifier was not found.
    #[error("cmd {cmd_ix} not found")]
    MissingCmd { cmd_ix: CmdIx<Ix> },
    /// Slot for the supplied index was not found.
    #[error("slot {slot} not found on command {cmd_ix}")]
    MissingSlot { cmd_ix: CmdIx<Ix>, slot: usize },
    /// Connection for the supplied identifier was not found.
    #[error("conn {conn_ix} ")]
    MissingConn { conn_ix: ConnIx<Ix> },
    /// Connection was not created due to an existing connection
    /// at the given input slot.
    #[error("input slot {slot} already in use for command {cmd_ix}")]
    SlotTaken { cmd_ix: CmdIx<Ix>, slot: usize },
    /// Connection was not created due to a type mismatch.
    #[error("conn type mismatch: {0}")]
    ConnTyMismatch(#[from] ConnTyMismatch<Cmd>),
}

/// A graph of user defined [commands](GraphCmd) and [connections](cmd::Conn) between them.
pub struct CmdGraph<Cmd, Ix>
where
    Ix: IndexType,
{
    graph: InnerGraph<Cmd, Ix>,
}

/// Default implementation for creating an empty command graph.
impl<Cmd, Ix> Default for CmdGraph<Cmd, Ix>
where
    Ix: IndexType,
{
    /// Create an empty graph.
    fn default() -> Self {
        Self::with_capacity(0, 0)
    }
}

impl<Cmd, Ix> CmdGraph<Cmd, Ix>
where
    Ix: IndexType,
{
    /// Create an empty graph with pre-allocated space for [commands](GraphCmd) and [connections](cmd::Conn).
    pub fn with_capacity(cmds: usize, conns: usize) -> Self {
        // Pre-allocate space for output node.
        let graph = InnerGraph::with_capacity(cmds, conns);
        Self { graph }
    }

    /// Get access to the [`Cmd`](GraphCmd) for the given [identifier](CmdIx).
    pub fn cmd(&self, ix: CmdIx<Ix>) -> Option<&Cmd> {
        self.graph.node_weight(ix.into())
    }

    /// Get mutable access to the [`Cmd`](GraphCmd) for the given [identifier](CmdIx).
    pub fn cmd_mut(&mut self, ix: CmdIx<Ix>) -> Option<&mut Cmd> {
        self.graph.node_weight_mut(ix.into())
    }

    /// Add a [command](GraphCmd) to the graph.
    pub fn add_cmd(&mut self, cmd: impl Into<Cmd>) -> CmdIx<Ix> {
        let node_ix = self.graph.add_node(cmd.into());
        CmdIx(node_ix)
    }

    /// Remove a [command](GraphCmd) from the graph.
    ///
    /// Returns the removed command if it exists.
    pub fn remove_cmd(&mut self, node: CmdIx<Ix>) -> Option<Cmd> {
        self.graph.remove_node(node.into())
    }

    /// Get access to the [`Conn`](cmd::Conn) for the given [identifier](ConnIx).
    pub fn conn(&self, ix: ConnIx<Ix>) -> Option<&Conn> {
        self.graph.edge_weight(ix.into())
    }

    /// Get mutable access to the [`Conn`](cmd::Conn) for the given [identifier](ConnIx).
    pub fn conn_mut(&mut self, ix: ConnIx<Ix>) -> Option<&mut Conn> {
        self.graph.edge_weight_mut(ix.into())
    }

    /// Add a [connection](cmd::Conn) between two [commands](GraphCmd) to the graph.
    /// Does not check for type mismatch.
    fn add_conn_unchecked(&mut self, from: CmdIx<Ix>, to: CmdIx<Ix>, conn: Conn) -> ConnIx<Ix> {
        let edge_ix = self.graph.add_edge(from.into(), to.into(), conn);
        ConnIx(edge_ix)
    }

    /// Remove a [`Conn`](cmd::Conn) from the [`Graph`].
    ///
    /// Returns the removed connection if it exists.
    pub fn remove_conn(&mut self, conn: ConnIx<Ix>) -> Option<Conn> {
        self.graph.remove_edge(conn.into())
    }
}

impl<Cmd, Desc, Ix> CmdGraph<Cmd, Ix>
where
    Cmd: GraphCmd<Desc = Desc>,
    Desc: CmdDataDesc,
    Ix: IndexType,
{
    /// Get access to the [input slot](cmd::InputSlot) of a given [command](GraphCmd).
    pub fn input_slot(
        &self,
        cmd_ix: CmdIx<Ix>,
        slot: usize,
    ) -> Result<&InputSlot<Desc>, GraphError<Cmd, Ix>> {
        let cmd = self.cmd(cmd_ix).ok_or(GraphError::MissingCmd { cmd_ix })?;
        let cmd = cmd
            .input_slots()
            .get(slot)
            .ok_or(GraphError::MissingSlot { cmd_ix, slot })?;
        Ok(cmd)
    }

    /// Get access to the [output slot](cmd::OutputSlot) of a given [command](GraphCmd).
    pub fn output_slot(
        &self,
        cmd_ix: CmdIx<Ix>,
        slot: usize,
    ) -> Result<&OutputSlot<Desc>, GraphError<Cmd, Ix>> {
        let cmd = self.cmd(cmd_ix).ok_or(GraphError::MissingCmd { cmd_ix })?;
        let cmd = cmd
            .output_slots()
            .get(slot)
            .ok_or(GraphError::MissingSlot { cmd_ix, slot })?;
        Ok(cmd)
    }

    // Checks if a slot is empty.
    pub fn is_slot_taken(&self, cmd_ix: CmdIx<Ix>, slot: usize) -> bool {
        self.graph
            .edges_directed(cmd_ix.into(), petgraph::Direction::Incoming)
            .any(|e| e.weight().dest_slot == slot)
    }

    /// Add a [connection](cmd::Conn) between two [commands](GraphCmd) to the graph.
    /// Returns an error if the connection cannot be applied.
    pub fn add_conn(
        &mut self,
        from: CmdIx<Ix>,
        to: CmdIx<Ix>,
        conn: impl Into<Conn>,
    ) -> Result<ConnIx<Ix>, GraphError<Cmd, Ix>> {
        let conn: Conn = conn.into();
        // Check if target slot is empty.
        let slot_taken = self.is_slot_taken(to, conn.dest_slot);
        if slot_taken {
            return Err(GraphError::SlotTaken {
                cmd_ix: to,
                slot: conn.dest_slot,
            });
        }
        // Check for type mismatch.
        let mismatch = self.check_conn_ty_mismatch(from, to, &conn)?;
        if let Some(mismatch) = mismatch {
            return Err(mismatch.into());
        }
        // Add connection.
        Ok(self.add_conn_unchecked(from, to, conn))
    }

    /// Checks the entire graph for errors.
    ///
    /// Returns a list of [`ConnTyMismatch`] errors.
    /// The list is empty if no errors are found.
    ///
    /// May return a [`GraphError`] if the graph is otherwise malformed.
    pub fn find_errors(&self) -> Result<Vec<ConnTyMismatch<Cmd>>, GraphError<Cmd, Ix>> {
        let results = self
            .graph
            .edge_references()
            .map(|edge| {
                self.check_conn_ty_mismatch(
                    CmdIx(edge.source()),
                    CmdIx(edge.target()),
                    edge.weight(),
                )
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(results)
    }

    /// Check if a [connection](cmd::Conn) between two [commands](GraphCmd) has a type mismatch.
    ///
    /// Returns:
    /// - [`GraphError`] if the connection is impossible to apply for reasons other than type mismatch.
    /// - [`None`] if the connection is valid.
    /// - [`Some(ConnTyMismatch)`](ConnTyMismatch) if the connection has a type mismatch.
    fn check_conn_ty_mismatch(
        &self,
        from: CmdIx<Ix>,
        to: CmdIx<Ix>,
        conn: &Conn,
    ) -> Result<Option<ConnTyMismatch<Cmd>>, GraphError<Cmd, Ix>> {
        let src_slot = self.output_slot(from, conn.src_slot)?;
        let dest_slot = self.input_slot(to, conn.dest_slot)?;
        if src_slot.connectable_to(dest_slot) {
            Ok(None)
        } else {
            let mismatch = ConnTyMismatch {
                conn: conn.clone(),
                found_ty_desc: src_slot.ty_desc,
                expected_ty_descs: dest_slot.ty_descs.to_vec(),
            };
            Ok(Some(mismatch))
        }
    }
}
