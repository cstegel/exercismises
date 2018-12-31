use crate::node::Node;
use crate::simple_linked_list_into_iter::SimpleLinkedListIntoIter;
use crate::simple_linked_list_iter::SimpleLinkedListIter;

use std::iter::FromIterator;

/// singly linked list that acts like a stack
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn push(&mut self, element: T) {
        let mut new_node = Box::new(Node::new(element));
        new_node.next = self.head.take();
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.val)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => Some(&node.val),
            _ => None,
        }
    }

    pub fn iter(&self) -> SimpleLinkedListIter<T> {
        SimpleLinkedListIter { node: &self.head }
    }

    /// in-place reversal of list
    pub fn into_rev(mut self) -> Self {
        let mut rev_list = Self::new();
        while let Some(node) = self.pop() {
            rev_list.push(node);
        }
        rev_list
    }

    /// Generic Into<Container> impl but not as the actual Into trait because there
    /// would be a "coherence" issue with the Into<T, U: From<T>> implementation
    /// This is supposed to be solved by the "specialization" RFC:
    /// https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md
    /// An explanation is offerred here:
    /// https://www.reddit.com/r/rust/comments/87l21n/conflicting_implementation_when_implementing_from/dwdtluy
    pub fn into<Container>(self) -> Container
    where
        Container: FromIterator<T>,
    {
        self.into_iter().collect()
    }
}

/// methods only "Clone"able lists have
impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut rev_list = Self::new();
        for i in self.iter() {
            rev_list.push(i.clone());
        }
        rev_list
    }
}

/// Clone from a slice
impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(items: &[T]) -> Self {
        let mut list = SimpleLinkedList::new();
        for item in items {
            list.push(item.clone());
        }
        list
    }
}

/// Creates an IntoIterator that iterates by insertion order
/// Does O(n) operations to find end of the list when this is called
/// but each next() is O(1)
impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = SimpleLinkedListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let rev_list = self.into_rev();
        Self::IntoIter {
            node: rev_list.head,
        }
    }
}

/// Create from an iterator
impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for i in iter {
            list.push(i);
        }
        list
    }
}
