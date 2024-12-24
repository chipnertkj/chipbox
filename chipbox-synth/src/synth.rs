pub mod cmd;

use cmd::{ConnTyMismatch, GraphCmd};
use petgraph::{
    graph::{EdgeIndex, IndexType, NodeIndex},
    stable_graph::StableDiGraph,
};
use std::ops::Not;

type InnerGraph<C, Ix> = StableDiGraph<cmd::Cmd<C>, cmd::Conn<Ix>, Ix>;

/// An identifier for a [command](cmd::Cmd) in a [`Graph`].
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display)]
#[display("node/{:?}", self.0.index())]
pub struct CmdIx<Ix: IndexType>(NodeIndex<Ix>);

impl<Ix: IndexType> CmdIx<Ix> {
    /// Get access to the [`Cmd`](cmd::Cmd) for this identifier from the [`Graph`].
    pub fn cmd<C: GraphCmd>(self, synth: &Graph<C, Ix>) -> Option<&cmd::Cmd<C>> {
        synth.cmd(self)
    }

    /// Get mutable access to the [`Cmd`](cmd::Cmd) for this identifier from the [`Graph`].
    pub fn cmd_mut<C: GraphCmd>(self, synth: &mut Graph<C, Ix>) -> Option<&mut cmd::Cmd<C>> {
        synth.cmd_mut(self)
    }
}

/// An identifier for a [connection](cmd::Conn) between two [commands](cmd::Cmd) in a [`Graph`].
#[derive(Debug, Clone, Copy, derive_more::Into, derive_more::Display)]
#[display("edge/{:?}", self.0.index())]
pub struct ConnIx<Ix: IndexType>(EdgeIndex<Ix>);

impl<Ix: IndexType> ConnIx<Ix> {
    /// Get access to the [`Conn`](cmd::Conn) for this identifier from the [`Graph`].
    pub fn conn<C: GraphCmd>(self, synth: &Graph<C, Ix>) -> Option<&cmd::Conn<Ix>> {
        synth.conn(self)
    }

    /// Get mutable access to the [`Conn`](cmd::Conn) for this identifier from the [`Graph`].
    pub fn conn_mut<C: GraphCmd>(self, synth: &mut Graph<C, Ix>) -> Option<&mut cmd::Conn<Ix>> {
        synth.conn_mut(self)
    }
}

/// A graph of [commands](cmd::Cmd) and [connections](cmd::Conn) between them.
pub struct Graph<C: GraphCmd, Ix: IndexType> {
    graph: InnerGraph<C, Ix>,
}

/// Errors that can occur when executing operations on a [`Graph`].
#[derive(Debug, thiserror::Error)]
pub enum GraphError<Ix: IndexType> {
    /// Command for the supplied identifier was not found.
    #[error("cmd {ix} not found")]
    MissingCmd { ix: CmdIx<Ix> },
    /// Connection for the supplied identifier was not found.
    #[error("conn {ix} ")]
    MissingConn { ix: ConnIx<Ix> },
    /// Slot for the supplied index was not found.
    #[error("slot {slot} not found on command {node_ix}")]
    MissingSlot { node_ix: CmdIx<Ix>, slot: usize },
    /// Connection cannot be created due to a type mismatch.
    #[error("conn type mismatch: {0}")]
    ConnTyMismatch(#[from] ConnTyMismatch<Ix>),
}

/// Default implementation for creating an empty graph.
impl<C: GraphCmd, Ix: IndexType> Default for Graph<C, Ix> {
    /// Create an empty graph.
    fn default() -> Self {
        Self::with_capacity(0, 0)
    }
}

impl<C: GraphCmd, Ix: IndexType> Graph<C, Ix> {
    /// Create an empty graph with pre-allocated space for [commands](cmd::Cmd) and [connections](cmd::Conn).
    pub fn with_capacity(cmds: usize, conns: usize) -> Self {
        // Pre-allocate space for output node.
        let graph = InnerGraph::with_capacity(cmds, conns);
        Self { graph }
    }

    /// Get access to the [`Cmd`](cmd::Cmd) for the given [identifier](CmdIx).
    pub fn cmd(&self, ix: CmdIx<Ix>) -> Option<&cmd::Cmd<C>> {
        self.graph.node_weight(ix.into())
    }

    /// Get mutable access to the [`Cmd`](cmd::Cmd) for the given [identifier](CmdIx).
    pub fn cmd_mut(&mut self, ix: CmdIx<Ix>) -> Option<&mut cmd::Cmd<C>> {
        self.graph.node_weight_mut(ix.into())
    }

    /// Add a [command](cmd::Cmd) to the graph.
    pub fn add_cmd(&mut self, cmd: impl Into<cmd::Cmd<C>>) -> CmdIx<Ix> {
        let node_ix = self.graph.add_node(cmd.into());
        CmdIx(node_ix)
    }

    /// Add a [custom command](cmd::CustomCmd) to the graph.
    pub fn add_custom_cmd(&mut self, custom_cmd: impl Into<C>) -> CmdIx<Ix> {
        let custom_cmd = cmd::CustomCmd::from(custom_cmd.into());
        self.add_cmd(custom_cmd)
    }

    /// Remove a [command](cmd::Cmd) from the graph.
    ///
    /// Returns the removed command if it exists.
    pub fn remove_cmd(&mut self, node: CmdIx<Ix>) -> Option<cmd::Cmd<C>> {
        self.graph.remove_node(node.into())
    }

    /// Get access to the [`Conn`](cmd::Conn) for the given [identifier](ConnIx).
    pub fn conn(&self, ix: ConnIx<Ix>) -> Option<&cmd::Conn<Ix>> {
        self.graph.edge_weight(ix.into())
    }

    /// Get mutable access to the [`Conn`](cmd::Conn) for the given [identifier](ConnIx).
    pub fn conn_mut(&mut self, ix: ConnIx<Ix>) -> Option<&mut cmd::Conn<Ix>> {
        self.graph.edge_weight_mut(ix.into())
    }

    /// Add a [connection](cmd::Conn) between two [commands](cmd::Cmd) to the graph.
    /// Returns an error if the connection cannot be applied.
    pub fn add_conn(&mut self, conn: cmd::Conn<Ix>) -> Result<ConnIx<Ix>, GraphError<Ix>> {
        let mismatch = self.check_conn_ty_mismatch(&conn)?;
        mismatch
            .map(Into::into)
            .map(Err)
            .unwrap_or_else(|| Ok(self.add_conn_unchecked(conn)))
    }

    /// Add a [connection](cmd::Conn) between two [commands](cmd::Cmd) to the graph.
    /// Does not check for type mismatch.
    fn add_conn_unchecked(&mut self, conn: cmd::Conn<Ix>) -> ConnIx<Ix> {
        let edge_ix = self
            .graph
            .add_edge(conn.src_cmd.into(), conn.dest_cmd.into(), conn);
        ConnIx(edge_ix)
    }

    /// Remove a [`Conn`](cmd::Conn) from the [`Graph`].
    ///
    /// Returns the removed connection if it exists.
    pub fn remove_conn(&mut self, conn: ConnIx<Ix>) -> Option<cmd::Conn<Ix>> {
        self.graph.remove_edge(conn.into())
    }

    /// Checks the entire graph for errors.
    ///
    /// Returns a list of [`ConnTyMismatch`] errors.
    /// The list is empty if no errors are found.
    ///
    /// May return a [`GraphError`] if the graph is otherwise malformed.
    pub fn find_errors(&self) -> Result<Vec<ConnTyMismatch<Ix>>, GraphError<Ix>> {
        let results = self
            .graph
            .edge_weights()
            .map(|conn| self.check_conn_ty_mismatch(conn))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(results)
    }

    /// Check if a [connection](cmd::Conn) between two [commands](cmd::Cmd) has a type mismatch.
    fn check_conn_ty_mismatch(
        &self,
        conn: &cmd::Conn<Ix>,
    ) -> Result<Option<cmd::ConnTyMismatch<Ix>>, GraphError<Ix>> {
        // Access source and destination commands and slots.
        let src_cmd = conn
            .src_cmd(self)
            .ok_or(GraphError::MissingCmd { ix: conn.src_cmd })?;
        let src_slot = src_cmd
            .output_slot(conn.src_slot)
            .ok_or(GraphError::MissingSlot {
                node_ix: conn.src_cmd,
                slot: conn.src_slot,
            })?;
        let dest_cmd = conn
            .dest_cmd(self)
            .ok_or(GraphError::MissingCmd { ix: conn.dest_cmd })?;
        let dest_slot = dest_cmd
            .input_slot(conn.dest_slot)
            .ok_or(GraphError::MissingSlot {
                node_ix: conn.dest_cmd,
                slot: conn.dest_slot,
            })?;
        // Check for signal type matches.
        // Only if source is a signal.
        if let cmd::SlotTy::Signal {
            bind_ch_n: src_ch_n,
        } = src_slot.ty
        {
            let valid = dest_slot
                .tys
                .iter()
                // Check if any destination slot is valid.
                .any(|ty| match ty {
                    cmd::SlotTy::Signal {
                        bind_ch_n: dest_ch_n,
                    } => match (src_ch_n, dest_ch_n) {
                        // Unbounded to unbounded is always valid.
                        (0, 0) => true,
                        // Unbounded to bounded is not valid.
                        // Bounded expects the same number of channels.
                        (0, _) => false,
                        // Bounded to unbounded is valid.
                        // Unbounded doesn't pose restrictions.
                        (_, 0) => true,
                        // Otherwise, just check if the number of channels matches.
                        // This may let us exit early
                        _ => src_ch_n == *dest_ch_n,
                    },
                    _ => false,
                });
            if valid {
                return Ok(None);
            }
        }
        // Check if slot types mismatch otherwise.
        let mismatch_opt = cmd::Cmd::<C>::slot_ty_matches(src_slot, dest_slot)
            .not()
            .then(|| cmd::ConnTyMismatch {
                conn: conn.clone(),
                found_ty: src_slot.ty,
                expected_tys: dest_slot.tys.to_vec(),
            })
            .or(None);
        Ok(mismatch_opt)
    }
}

#[cfg(test)]
mod tests {
    use cmd::SlotTy;
    use cmd::{InputSlot, OutputSlot};

    use super::*;

    #[enum_dispatch::enum_dispatch(GraphCmd)]
    #[derive(enum_as_inner::EnumAsInner)]
    enum Custom {
        Foo,
    }

    struct Foo {
        input: InputSlot,
        output: OutputSlot,
    }

    impl Foo {
        fn new(chs_in: usize, chs_out: usize) -> Self {
            let output = OutputSlot {
                name: "".into(),
                ty: SlotTy::Signal { bind_ch_n: chs_out },
            };
            let input = InputSlot {
                name: "".into(),
                tys: vec![SlotTy::Signal { bind_ch_n: chs_in }],
            };
            Self { input, output }
        }

        fn set_input_chs(&mut self, bind_ch_n: usize) {
            self.input.tys[0] = SlotTy::Signal { bind_ch_n };
        }
    }

    impl GraphCmd for Foo {
        fn input_slots(&self) -> &[InputSlot] {
            std::slice::from_ref(&self.input)
        }
        fn output_slots(&self) -> &[OutputSlot] {
            std::slice::from_ref(&self.output)
        }
    }

    /// Adding a connection with a type mismatch returns an error.
    /// The connection is made between two custom commands.
    #[test]
    fn custom_ty_mismatch() {
        let mut synth = Graph::<Custom, u32>::with_capacity(2, 1);
        let foo = synth.add_custom_cmd(Foo::new(0, 1));
        let bar = synth.add_custom_cmd(Foo::new(2, 0));
        let conn = cmd::Conn::new((foo, 0), (bar, 0));
        match synth.add_conn(conn) {
            Err(GraphError::ConnTyMismatch(_)) => (),
            Err(e) => panic!("expected ty mismatch, got {e:?}"),
            Ok(c) => panic!("expected error, got ok: {}", c.conn(&synth).unwrap()),
        }
    }

    /// Adding a connection from a bounded signal to an unbounded signal does not return an error.
    /// The connection is made between two custom commands.
    #[test]
    fn custom_no_ty_mismatch() {
        let mut synth = Graph::<Custom, u32>::with_capacity(2, 1);
        let foo = synth.add_custom_cmd(Foo::new(0, 1));
        let bar = synth.add_custom_cmd(Foo::new(1, 0));
        let conn = cmd::Conn::new((foo, 0), (bar, 0));
        let _ = synth.add_conn(conn).expect("expected ok");
        assert!(synth.find_errors().unwrap().is_empty());
        bar.cmd_mut(&mut synth)
            .unwrap()
            .as_custom_mut()
            .unwrap()
            .as_mut()
            .as_foo_mut()
            .unwrap()
            .set_input_chs(2);
        assert_eq!(synth.find_errors().unwrap().len(), 1);
    }
}
