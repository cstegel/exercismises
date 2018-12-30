pub mod graph_items;

use crate::graph::graph_items::edge::Edge;
use crate::graph::graph_items::node::Node;

use std::collections::HashMap;

fn attrs_slice_to_map<T: AsRef<str>>(attrs: &[(T, T)]) -> HashMap<String, String> {
    attrs
        .iter()
        .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
        .collect()
}

#[derive(PartialEq, Debug, Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: vec![],
            edges: vec![],
            attrs: HashMap::new(),
        }
    }

    pub fn get_node(&self, name: &str) -> Option<&Node> {
        self.nodes.iter().find(|n| n.name == name)
    }

    pub fn with_attrs<T: AsRef<str>>(mut self, attrs: &[(T, T)]) -> Self {
        self.attrs = crate::graph::attrs_slice_to_map(attrs);
        self
    }

    pub fn with_nodes(mut self, nodes: &[Node]) -> Self {
        self.nodes = nodes.iter().cloned().collect();
        self
    }

    pub fn with_edges(mut self, edges: &[Edge]) -> Self {
        self.edges = edges.iter().cloned().collect();
        self
    }
}
