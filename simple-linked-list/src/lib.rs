use std::iter::FromIterator;
use std::marker::PhantomData;

type NodeLink<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: NodeLink<T>,
}

impl<T> Node<T> {
    fn new(data: T, next: NodeLink<T>) -> Self {
        Node { data, next }
    }
}

pub struct SimpleLinkedList<T> {
    head: NodeLink<T>,
    // dummy is needed to avoid unused parameter error during compilation
    dummy: PhantomData<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            dummy: PhantomData,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut size = 0;
        let mut temp_head = self.head.as_ref();
        while let Some(node) = temp_head {
            size += 1;
            temp_head = node.next.as_ref();
        }
        size
    }

    // Always inserted in the head
    pub fn push(&mut self, element: T) {
        let new = Node::new(element, self.head.take());
        self.head = Some(Box::new(new));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            head.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head| &head.data)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut head = self.head;
        let mut temp_vec = vec![];
        while let Some(node) = head {
            temp_vec.push(node.data);
            head = node.next;
        }
        let mut list = SimpleLinkedList::new();
        for v in temp_vec {
            list.push(v);
        }
        list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        let mut iter = iter.into_iter();
        while let Some(v) = iter.next() {
            list.push(v)
        }

        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = vec![];

        while let Some(data) = linked_list.pop() {
            v.insert(0, data);
        }

        v
    }
}
