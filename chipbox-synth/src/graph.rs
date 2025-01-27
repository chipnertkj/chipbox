#![allow(dead_code)]

use petgraph::graph::{DiGraph, NodeIndex};
use slotmap::SlotMap;

mod data;
mod edge;
mod node;
mod op;

struct Graph {
    id_graph: DiGraph<node::NodeId, ()>,
    nodes: SlotMap<node::NodeId, node::Node>,
}

/// Construction.
impl Graph {
    pub fn new() -> Self {
        Self {
            id_graph: DiGraph::new(),
            nodes: SlotMap::with_key(),
        }
    }

    pub fn with_capacity(nodes: usize) -> Self {
        Self {
            id_graph: DiGraph::with_capacity(nodes, 0),
            nodes: SlotMap::with_capacity_and_key(nodes),
        }
    }
}

/// Rendering.
impl Graph {
    pub fn render_pipeline<const FRAME_COUNT: usize>(&self) -> RenderPipeline<FRAME_COUNT> {
        RenderPipeline::new(self)
    }
}

struct RenderPipeline<'graph, const FRAME_COUNT: usize> {
    graph: &'graph Graph,
    steps: Vec<Vec<op::Op>>,
    dataset: data::Dataset<FRAME_COUNT>,
}

impl<'graph, const FRAME_COUNT: usize> RenderPipeline<'graph, FRAME_COUNT> {
    /// This function takes a node graph and returns a collection of steps.
    /// A step is a set of operations that may be executed in parallel.
    /// Every node in a graph contains a command that can be converted into an executable operation.
    fn build_steps(graph: &'graph Graph) -> (Vec<Vec<op::Op>>, data::Dataset<FRAME_COUNT>) {
        let (audio_blocks, signal_blocks, trigger_blocks) = (0, 0, 0);
        let dataset = data::Dataset::<FRAME_COUNT>::with_capacity(
            audio_blocks,
            signal_blocks,
            trigger_blocks,
        );
        (vec![], dataset)
    }

    pub fn new(graph: &'graph Graph) -> Self {
        let (steps, dataset) = Self::build_steps(graph);
        Self {
            graph,
            steps,
            dataset,
        }
    }

    pub fn render(&self, output: &mut [f32]) {}
}

/// Node management.
impl Graph {
    fn find_node_ix(&self, id: node::NodeId) -> Option<NodeIndex> {
        self.id_graph
            .node_indices()
            .find(|ix| self.id_graph[*ix] == id)
    }

    pub fn nodes(&self) -> impl Iterator<Item = &node::Node> {
        self.nodes.values()
    }

    pub fn node_ids(&self) -> impl Iterator<Item = node::NodeId> {
        self.id_graph.node_indices().map(|ix| self.id_graph[ix])
    }

    pub fn node(&self, id: node::NodeId) -> Option<&node::Node> {
        self.nodes.get(id)
    }

    pub fn add_node(&mut self, node: node::Node) -> node::NodeId {
        let node_id = self.nodes.insert(node);
        self.id_graph.add_node(node_id);
        node_id
    }

    pub fn remove_node(&mut self, id: node::NodeId) -> Option<node::Node> {
        let node_ix = self.find_node_ix(id)?;
        self.id_graph.remove_node(node_ix);
        self.nodes.remove(id)
    }
}
