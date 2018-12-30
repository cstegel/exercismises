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
