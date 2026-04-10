use crate::config::ProxyNode;

pub struct NodeManager;

impl NodeManager {
    pub fn new() -> Self {
        Self
    }

    pub fn list_nodes(&self, nodes: &[ProxyNode]) {
        for node in nodes {
            println!("{}: {}:{} [{}]", node.name, node.server, node.port, node.protocol);
        }
    }

    pub fn test_node(&self, node: &ProxyNode) -> bool {
        println!("Testing node: {}", node.name);
        true
    }
}
