use super::node::NodeId;

slotmap::new_key_type! {
    pub struct EdgeId;
}

pub struct Edge {
    pub src_node: NodeId,
    pub src_port: usize,
    pub dst_node: NodeId,
    pub dst_port: usize,
}

impl Edge {
    pub fn new(
        (src_node, src_port): (NodeId, usize),
        (dst_node, dst_port): (NodeId, usize),
    ) -> Self {
        Self {
            src_node,
            dst_node,
            src_port,
            dst_port,
        }
    }
}
