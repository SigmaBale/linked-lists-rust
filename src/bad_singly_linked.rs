#![allow(dead_code)]
use std::mem;

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            element: value,
            next: self.pop_node(),
        });
        self.head = Link::Element(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        if let Link::Element(node) = self.pop_node() {
            self.head = node.next;
            Some(node.element)
        } else {
            None
        }
    }

    fn pop_node(&mut self) -> Link {
        mem::replace(&mut self.head, Link::Empty)
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while let Link::Element(boxed_node) = self.pop_node() {
            self.head = boxed_node.next;
        }
    }
}

enum Link {
    Empty,
    Element(Box<Node>),
}

struct Node {
    element: i32,
    next: Link,
}

#[cfg(test)]
mod test {

    use super::List;

    #[test]
    fn test_basics() {
        let mut list = List::new();

        // populating list
        list.push(5);
        list.push(4);
        list.push(3);

        // normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        list.push(1);
        list.pop();

        assert_eq!(list.pop(), Some(5));

        // when list is empty
        assert_eq!(list.pop(), None);

        list.push(2);
        list.push(3);
        list.push(4);
    }
}
