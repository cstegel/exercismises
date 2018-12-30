pub mod graph {
    use std::collections::HashMap;

    fn attrs_slice_to_map<T: AsRef<str>>(attrs: &[(T, T)]) -> HashMap<String, String> {
        attrs
            .iter()
            .map(|(k, v)| (k.as_ref().to_string(), v.as_ref().to_string()))
            .collect()
    }

    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(PartialEq, Debug, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs<T: AsRef<str>>(mut self, attrs: &[(T, T)]) -> Node {
                    self.attrs = crate::graph::attrs_slice_to_map(attrs);
                    self
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    self.attrs.get(name).map(String::as_str)
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                pub start: String,
                pub end: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Edge {
                    Edge {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs<T: AsRef<str>>(mut self, attrs: &[(T, T)]) -> Edge {
                    self.attrs = crate::graph::attrs_slice_to_map(attrs);
                    self
                }
            }
        }
    }

    use crate::graph::graph_items::edge::Edge;
    use crate::graph::graph_items::node::Node;

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

}
