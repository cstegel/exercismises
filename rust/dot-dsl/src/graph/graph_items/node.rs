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
