#![allow(dead_code)]
use std::mem;
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::fmt::{Debug, Result, Formatter};
use std::cmp::Ordering;

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

    pub fn front(&self) -> Option<&T> {
        unsafe { 
            Some(&(*self.front?.as_ptr()).elem)
        }
    }
    
    pub fn back(&self) -> Option<&T> {
        unsafe { 
            Some(&(*self.back?.as_ptr()).elem)
        }
    }
    
    pub fn front_mut(&self) -> Option<&mut T> {
        unsafe {
            Some(&mut(*self.front?.as_ptr()).elem)
        }
    }

    pub fn back_mut(&self) -> Option<&mut T> {
        unsafe {
            Some(&mut(*self.back?.as_ptr()).elem)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { 
            list: self 
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { 
            front: self.front, 
            back: self.back, 
            len: self.len, 
            _marker: PhantomData 
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { 
            front: self.front, 
            back: self.back, 
            len: self.len, 
            _marker: PhantomData 
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        while let Some(_) = self.pop_back() {}
    }

    pub fn cursor_mut(&mut self) -> CursorMut<T> {
        CursorMut { node: None, list: self, index: None }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _marker: PhantomData<&'a T>
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.front = (*node.as_ptr()).back;
                self.len -= 1;
                &(*node.as_ptr()).elem
            })
        }else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.back = (*node.as_ptr()).front;
                self.len -= 1;
                &(*node.as_ptr()).elem
            })
        }else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _marker: PhantomData<&'a mut T>
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.front = (*node.as_ptr()).back;
                self.len -= 1;
                &mut (*node.as_ptr()).elem
            })
        }else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.back = (*node.as_ptr()).front;
                self.len -= 1;
                &mut (*node.as_ptr()).elem
            })
        }else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        for item in self {
            new_list.push_back(item.clone());
        }
        new_list
    }
}

impl<T> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item);
        }
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut new_list = Self::new();
        new_list.extend(iter);
        new_list
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_list().entries(self).finish()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().eq(other)
    }
    fn ne(&self, other: &Self) -> bool {
        self.len != other.len || self.iter().ne(other) 
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}

impl<T: Hash> Hash for LinkedList<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for item in self {
            item.hash(state);
        }
    }
}

unsafe impl<T: Send> Send for LinkedList<T> {}
unsafe impl<T: Sync> Sync for LinkedList<T> {}

unsafe impl<T: Send> Send for Iter<'_, T> {}
unsafe impl<T: Sync> Sync for Iter<'_, T> {}

unsafe impl<T: Send> Send for IterMut<'_, T> {}
unsafe impl<T: Sync> Sync for IterMut<'_, T> {}

pub struct CursorMut<'a, T> {
    node: Link<T>,
    list: &'a mut LinkedList<T>,
    index: Option<usize>
}

impl<T> CursorMut<'_, T> {
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn move_next(&mut self) {
        if let Some(node) = self.node {
            unsafe { 
                self.node = (*node.as_ptr()).back 
            }
            if self.node.is_some() {
                *self.index.as_mut().unwrap() += 1;
            }else {
                self.index = None;
            }
        }else if !self.list.is_empty() {
            self.node = self.list.front;
            self.index = Some(0);
        }else {
            return;
        }
    }

    pub fn move_back(&mut self) {
        if let Some(node) = self.node {
            unsafe { 
                self.node = (*node.as_ptr()).front 
            }
            if self.node.is_some() {
                *self.index.as_mut().unwrap() -= 1;
            }else {
                self.index = None;
            }
        }else if !self.list.is_empty() {
            self.node = self.list.back;
            self.index = Some(self.list.len - 1);
        }else {
            return;
        }
    }

    pub fn current(&mut self) -> Option<&mut T> {
        self.node.map(|node| unsafe { &mut(*node.as_ptr()).elem })
    }

    pub fn peek_next(&mut self) -> Option<&mut T> {
        self.node.and_then(|node| 
            unsafe {
                (*node.as_ptr()).back.map(|node| &mut (*node.as_ptr()).elem)
            })
    }

    pub fn peek_prev(&mut self) -> Option<&mut T> {
        self.node.and_then(|node| 
            unsafe {
                (*node.as_ptr()).front.map(|node| &mut (*node.as_ptr()).elem)
            })
    }

    pub fn split_before(&mut self) -> LinkedList<T> {
        if let Some(current) = self.node {    
            unsafe {
                if (*current.as_ptr()).front == None {
                    return LinkedList::new()
                }

                let old_len = self.list.len;
                let old_front = self.list.front;

                self.list.len -= self.index.unwrap();
                self.list.front = Some(current);
                self.index = Some(0);

                let new_back = (*current.as_ptr()).front.take();
                (*new_back.unwrap().as_ptr()).back = None;

                LinkedList {
                    front: old_front,
                    back: new_back,
                    len: old_len - self.list.len,
                    _marker: PhantomData
                }
            }
        }else {
            mem::replace(self.list, LinkedList::new())
        }
    }

    pub fn split_after(&mut self) -> LinkedList<T> {
        if let Some(current) = self.node {    
            unsafe {
                if (*current.as_ptr()).back == None {
                    return LinkedList::new()
                }

                let old_len = self.list.len;
                let old_back = self.list.back;

                self.list.len -= self.index.unwrap();
                self.list.back = Some(current);
                self.index = Some(self.list.len - 1);

                let new_front = (*current.as_ptr()).back.take();
                (*new_front.unwrap().as_ptr()).front = None;

                LinkedList {
                    front: new_front,
                    back: old_back,
                    len: old_len - self.list.len,
                    _marker: PhantomData
                }
            }
        }else {
            mem::replace(self.list, LinkedList::new())
        }
    }
}

/// Doc test that will fail in order to prove that `IterMut<'_, T>` is invariant,
/// meaning we can't change the lifetime of our type `T` because IterMut<'_, T> "contains" `&mut`
/// reference to `T` as marked with `_marker` field (`PhantomData<&mut T>`) making our `IterMut` type invariant.
///
/// ```compile_fail
/// use linked_lists_rust::production_queue_unsafe::IterMut;
/// 
/// fn iter_mut_covariant<'i, 'a, T>(x: IterMut<'i, &'static T>) -> IterMut<'i, &'a T> { x }
/// ```
fn iter_mut_invariant() {}

#[cfg(test)]
mod test {
    use super::LinkedList;

    fn generate_test() -> LinkedList<i32> {
        list_from(&[0, 1, 2, 3, 4, 5, 6])
    }

    fn list_from<T: Clone>(v: &[T]) -> LinkedList<T> {
        v.iter().map(|x| (*x).clone()).collect()
    }

    #[test]
    fn test_basic_front() {
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

    #[test]
    fn test_basic() {
        let mut m = LinkedList::new();
        assert_eq!(m.pop_front(), None);
        assert_eq!(m.pop_back(), None);
        assert_eq!(m.pop_front(), None);
        m.push_front(1);
        assert_eq!(m.pop_front(), Some(1));
        m.push_back(2);
        m.push_back(3);
        assert_eq!(m.len(), 2);
        assert_eq!(m.pop_front(), Some(2));
        assert_eq!(m.pop_front(), Some(3));
        assert_eq!(m.len(), 0);
        assert_eq!(m.pop_front(), None);
        m.push_back(1);
        m.push_back(3);
        m.push_back(5);
        m.push_back(7);
        assert_eq!(m.pop_front(), Some(1));

        let mut n = LinkedList::new();
        n.push_front(2);
        n.push_front(3);
        {
            assert_eq!(n.front().unwrap(), &3);
            let x = n.front_mut().unwrap();
            assert_eq!(*x, 3);
            *x = 0;
        }
        {
            assert_eq!(n.back().unwrap(), &2);
            let y = n.back_mut().unwrap();
            assert_eq!(*y, 2);
            *y = 1;
        }
        assert_eq!(n.pop_front(), Some(0));
        assert_eq!(n.pop_front(), Some(1));
    }

    #[test]
    fn test_iterator() {
        let m = generate_test();
        for (i, elt) in m.iter().enumerate() {
            assert_eq!(i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_iterator_double_end() {
        let mut n = LinkedList::new();
        assert_eq!(n.iter().next(), None);
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(it.next().unwrap(), &6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next_back().unwrap(), &4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next_back().unwrap(), &5);
        assert_eq!(it.next_back(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_rev_iter() {
        let m = generate_test();
        for (i, elt) in m.iter().rev().enumerate() {
            assert_eq!(6 - i as i32, *elt);
        }
        let mut n = LinkedList::new();
        assert_eq!(n.iter().rev().next(), None);
        n.push_front(4);
        let mut it = n.iter().rev();
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next().unwrap(), &4);
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_mut_iter() {
        let mut m = generate_test();
        let mut len = m.len();
        for (i, elt) in m.iter_mut().enumerate() {
            assert_eq!(i as i32, *elt);
            len -= 1;
        }
        assert_eq!(len, 0);
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next().is_none());
        n.push_front(4);
        n.push_back(5);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert!(it.next().is_some());
        assert!(it.next().is_some());
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert!(it.next().is_none());
    }

    #[test]
    fn test_iterator_mut_double_end() {
        let mut n = LinkedList::new();
        assert!(n.iter_mut().next_back().is_none());
        n.push_front(4);
        n.push_front(5);
        n.push_front(6);
        let mut it = n.iter_mut();
        assert_eq!(it.size_hint(), (3, Some(3)));
        assert_eq!(*it.next().unwrap(), 6);
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(*it.next_back().unwrap(), 4);
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(*it.next_back().unwrap(), 5);
        assert!(it.next_back().is_none());
        assert!(it.next().is_none());
    }

    #[test]
    fn test_eq() {
        let mut n: LinkedList<u8> = list_from(&[]);
        let mut m = list_from(&[]);
        assert!(n == m);
        n.push_front(1);
        assert!(n != m);
        m.push_back(1);
        assert!(n == m);

        let n = list_from(&[2, 3, 4]);
        let m = list_from(&[1, 2, 3]);
        assert!(n != m);
    }

    #[test]
    fn test_ord() {
        let n = list_from(&[]);
        let m = list_from(&[1, 2, 3]);
        assert!(n < m);
        assert!(m > n);
        assert!(n <= n);
        assert!(n >= n);
    }

    #[test]
    fn test_ord_nan() {
        let nan = 0.0f64 / 0.0;
        let n = list_from(&[nan]);
        let m = list_from(&[nan]);
        assert!(!(n < m));
        assert!(!(n > m));
        assert!(!(n <= m));
        assert!(!(n >= m));

        let n = list_from(&[nan]);
        let one = list_from(&[1.0f64]);
        assert!(!(n < one));
        assert!(!(n > one));
        assert!(!(n <= one));
        assert!(!(n >= one));

        let u = list_from(&[1.0f64, 2.0, nan]);
        let v = list_from(&[1.0f64, 2.0, 3.0]);
        assert!(!(u < v));
        assert!(!(u > v));
        assert!(!(u <= v));
        assert!(!(u >= v));

        let s = list_from(&[1.0f64, 2.0, 4.0, 2.0]);
        let t = list_from(&[1.0f64, 2.0, 3.0, 2.0]);
        assert!(!(s < t));
        assert!(s > one);
        assert!(!(s <= one));
        assert!(s >= one);
    }

    #[test]
    fn test_debug() {
        let list: LinkedList<i32> = (0..10).collect();
        assert_eq!(format!("{:?}", list), "[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]");

        let list: LinkedList<&str> = vec!["just", "one", "test", "more"]
            .iter().copied()
            .collect();
        assert_eq!(format!("{:?}", list), r#"["just", "one", "test", "more"]"#);
    }

    #[test]
    fn test_hashmap() {
        // Check that HashMap works with this as a key

        let list1: LinkedList<i32> = (0..10).collect();
        let list2: LinkedList<i32> = (1..11).collect();
        let mut map = std::collections::HashMap::new();

        assert_eq!(map.insert(list1.clone(), "list1"), None);
        assert_eq!(map.insert(list2.clone(), "list2"), None);

        assert_eq!(map.len(), 2);

        assert_eq!(map.get(&list1), Some(&"list1"));
        assert_eq!(map.get(&list2), Some(&"list2"));

        assert_eq!(map.remove(&list1), Some("list1"));
        assert_eq!(map.remove(&list2), Some("list2"));

        assert!(map.is_empty());
    }
}