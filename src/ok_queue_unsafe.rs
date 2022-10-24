#![allow(dead_code)]
use std::marker::PhantomData;
use std::ptr;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = *mut Node<T>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::into_raw(Box::new(Node {
            elem,
            next: ptr::null_mut(),
        }));

        if !self.tail.is_null() {
            unsafe { (*self.tail).next = new_tail }
        } else {
            self.head = new_tail
        }

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if !self.head.is_null() {
            unsafe {
                let old_head = Box::from_raw(self.head);
                self.head = old_head.next;

                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(old_head.elem)
            }
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&'_ T> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&(*self.head).elem) }
        }
    }

    pub fn peek_mut(&mut self) -> Option<&'_ mut T> {
        if self.head.is_null() {
            None
        } else {
            unsafe { Some(&mut (*self.head).elem) }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head,
            _marker: PhantomData,
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for List<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { next: self }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

pub struct IntoIter<T> {
    next: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    next: *mut Node<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.as_ref().map(|node| {
                self.next = node.next;
                &node.elem
            })
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    next: *mut Node<T>,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.as_mut().map(|node| {
                self.next = node.next;
                &mut node.elem
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn test_basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        list.push(6);
        list.push(7);

        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        let mut iter = list.iter();

        // Check return values
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));

        // Check exhaustion
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        let mut iter_mut = list.iter_mut();

        // Check return values
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), Some(&mut 4));

        // Check exhaustion
        assert_eq!(iter_mut.next(), None);
        assert_eq!(iter_mut.next(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        let mut into_iter = list.into_iter();

        // Check return values
        assert_eq!(into_iter.next(), Some(1));
        assert_eq!(into_iter.next(), Some(2));
        assert_eq!(into_iter.next(), Some(3));
        assert_eq!(into_iter.next(), Some(4));

        // Check exhaustion
        assert_eq!(into_iter.next(), None);
        assert_eq!(into_iter.next(), None);
    }

    #[test]
    fn test_peek_and_peek_mut() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));

        list.pop();
        list.pop();
        list.pop();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
    }

    #[test]
    fn test_miri_mixup() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push(7);
    }
}
