use std::ptr::{self, NonNull};
use std::mem;
use std::marker::PhantomData;

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T
}

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _marker: PhantomData<T>
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { 
            front: None, 
            back: None, 
            len: 0 ,
            _marker: PhantomData
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem
            })));

            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            }else {
                self.front = Some(new);
            }

            self.back = Some(new);
        }

        self.len += 1;
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem
            })));

            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            }else {
                self.back = Some(new);
            }

            self.front = Some(new);
        }

        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|node| { 
                let old_back = Box::from_raw(node.as_ptr());

                self.back = old_back.front;

                if let Some(new) = self.back {
                    (*new.as_ptr()).back = None;
                }else {
                    self.front = None;
                }
                self.len -= 1;
                old_back.elem
            })
        }
    }
    
    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| { 
                let old_front = Box::from_raw(node.as_ptr());

                self.front = old_front.back;

                if let Some(new) = self.front {
                    (*new.as_ptr()).front = None;
                }else {
                    self.back = None;
                }
                self.len -= 1;
                old_front.elem
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_basic_front_production_queue_unsafe() {
        let mut list = LinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}