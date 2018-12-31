use crate::node::Node;

pub struct SimpleLinkedListIntoIter<T> {
    pub node: Option<Box<Node<T>>>,
}

impl<T> Iterator for SimpleLinkedListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if let Some(node) = self.node.take() {
            self.node = node.next;
            Some(node.val)
        } else {
            None
        }
    }
}
