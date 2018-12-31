use crate::node::Node;

pub struct SimpleLinkedListIter<'a, T> {
    pub node: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for SimpleLinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.node {
            Some(ref node) => {
                self.node = &node.next;
                Some(&node.val)
            }
            None => None,
        }
    }
}
