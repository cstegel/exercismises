pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Self {
            val: val,
            next: None,
        }
    }
}
