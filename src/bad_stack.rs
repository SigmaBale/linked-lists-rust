use std::mem;

pub struct List {
    head: Line
}

impl List {
    pub fn new() -> Self {
        List { head: Line::Empty }
    }

    pub fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            element: value,
            next: mem::replace(&mut self.head, Line::Empty)
        });
        self.head = Line::Element(new_node);
    }
    
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Line::Empty) {
            Line::Empty => None,
            Line::Element(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Line::Empty);
        while let Line::Element(mut boxed_node) = curr_link {
            curr_link = mem::replace(&mut boxed_node.next, Line::Empty);
        }
    }
}

enum Line {
    Empty,
    Element(Box<Node>)
}

struct Node {
    element: i32,
    next: Line
}


#[cfg(test)]
mod test {

    use super::List;

    #[test]
    fn basics() {
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
    }

}