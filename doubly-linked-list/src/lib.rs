use std::marker::PhantomData;
use std::ptr::null_mut;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

#[derive(Debug)]
struct Node<T> {
    data: T,
    prev: *mut Node<T>,
    next: *mut Node<T>,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            prev: null_mut(),
            next: null_mut(),
        }
    }
}

unsafe impl<T> Send for Node<T> {}
unsafe impl<T> Send for LinkedList<T> {}

unsafe impl<T> Sync for Node<T> {}
unsafe impl<T> Sync for LinkedList<T> {}

pub struct LinkedList<T> {
    size: usize,
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

pub struct Cursor<'a, T> {
    list: &'a mut LinkedList<T>,
    curr: *mut Node<T>,
}

pub struct Iter<'a, T> {
    next: *mut Node<T>,
    _flg: PhantomData<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            size: 0,
            head: null_mut(),
            tail: null_mut(),
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        Cursor {
            curr: self.head,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        Cursor {
            curr: self.tail,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head,
            _flg: PhantomData,
        }
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.curr.is_null() {
            None
        } else {
            unsafe { Some(&mut (*self.curr).data) }
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        unsafe {
            self.curr = (*self.curr).next;
        }
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        unsafe {
            self.curr = (*self.curr).prev;
        }
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.curr.is_null() {
            return None;
        }

        unsafe {
            let next = (*self.curr).next;
            let prev = (*self.curr).prev;
            let curr = Box::from_raw(self.curr);

            match (prev.is_null(), next.is_null()) {
                (true, true) => {
                    self.list.head = null_mut();
                    self.list.tail = null_mut();
                    self.curr = null_mut();
                }
                (true, false) => {
                    (*next).prev = null_mut();
                    self.list.head = next;
                    self.curr = next;
                }
                (false, true) => {
                    (*prev).next = null_mut();
                    self.list.tail = prev;
                    self.curr = prev;
                }
                (false, false) => {
                    (*next).prev = prev;
                    (*prev).next = next;
                    self.curr = next;
                }
            }

            self.list.size -= 1;
            Some(curr.data)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let node = Box::into_raw(Box::new(Node::new(element)));

        if self.curr.is_null() {
            self.list.head = node;
            self.list.tail = node;
        } else {
            unsafe {
                let next = (*self.curr).next;
                if !next.is_null() {
                    (*next).prev = node;
                } else {
                    // self.curr is the last node
                    self.list.tail = node;
                }
                (*node).next = next;
                (*node).prev = self.curr;
                (*self.curr).next = node;
            }
        }

        self.list.size += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let node = Box::into_raw(Box::new(Node::new(element)));

        if self.curr.is_null() {
            self.list.head = node;
            self.list.tail = node;
        } else {
            unsafe {
                let prev = (*self.curr).prev;
                (*node).prev = prev;
                if !prev.is_null() {
                    (*prev).next = node;
                } else {
                    // self.curr is the first node
                    self.list.head = node;
                }
                (*self.curr).prev = node;
                (*node).next = self.curr;
            }
        }

        self.list.size += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        let curr = self.next;
        if curr.is_null() {
            return None;
        }
        unsafe {
            self.next = (*curr).next;
            Some(&(*curr).data)
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}
