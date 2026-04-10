use crate::proxy::models::Proxy;

pub struct NodeManager;

impl NodeManager {
    pub fn new() -> Self {
        Self
    }

    pub fn list_nodes(&self, nodes: &[Proxy]) {
        for node in nodes {
            println!("{}: {}:{} [{}]", node.name, node.server, node.port, node.proxy_type);
        }
    }

    pub fn test_node(&self, node: &Proxy) -> bool {
        println!("Testing node: {}", node.name);
        true
    }
}
