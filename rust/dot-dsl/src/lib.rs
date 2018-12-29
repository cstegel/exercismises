pub mod graph {
    pub mod graph_items {
        pub mod node {
            pub struct Node {
                name: String,
                attrs: Vec<Attribute>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Node {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, val)| Attribute::new(&key, &val))
                        .collect();
                    self
                }
            }
        }
        pub mod edge {
            pub struct Edge {
                start: String,
                end: String,
                attrs: Vec<Attribute>,
            }

            impl Edge {
                pub fn new(start: &str, end: &str) -> Edge {
                    Edge {
                        start: start.to_string(),
                        end: end.to_string(),
                        attrs: vec![],
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Edge {
                    self.attrs = attrs
                        .iter()
                        .map(|(key, val)| Attribute::new(&key, &val))
                        .collect();
                    self
                }
            }
        }
    }
    pub struct Graph {
        nodes: Vec<Node>,
        edges: Vec<Edge>,
        attrs: Vec<Attribute>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: vec![],
                edges: vec![],
                attrs: vec![],
            }
        }

        pub fn with_attrs(mut self, attrs: Vec<Attribute>) -> Self {
            self.attrs = attrs;
            self
        }

        pub fn with_nodes(mut self, nodes: Vec<Node>) -> Self {
            self.nodes = nodes;
            self
        }

        pub fn with_edges(mut self, edges: Vec<Edge>) -> Self {
            self.edges = edges;
            self
        }
    }
}
