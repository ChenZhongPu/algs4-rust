//! # FIFO queue
//!
//! A queue implemented with linked list

use std::ptr;

type Link<T> = Option<Box<Node<T>>>;
struct Node<T> {
    item: T,
    next: Link<T>,
}
pub struct Queue<T> {
    first: Link<T>,
    // For one-element stack, both `first` and `last` would point
    // to the same one. One solution is to use `Rc<RefCell<Node<T>>>`.
    last: *mut Node<T>,
    n: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            first: None,
            last: ptr::null_mut(),
            n: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn enqueue(&mut self, t: T) {
        let mut new_last = Box::new(Node {
            item: t,
            next: None,
        });
        let raw_last: *mut _ = &mut *new_last;

        if !self.last.is_null() {
            unsafe {
                (*self.last).next = Some(new_last);
            }
        } else {
            self.first = Some(new_last);
        }
        self.last = raw_last;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.first.take().map(|head| {
            // let head = *head;
            self.first = head.next;
            if self.first.is_none() {
                self.last = ptr::null_mut();
            }
            head.item
        })
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        let mut cur_link = self.first.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

pub struct IntoIter<T>(Queue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.dequeue()
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.item
        })
    }
}

impl<T> Queue<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.first.as_deref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn en_de_queue() {
        let mut q = Queue::new();
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);
        assert_eq!(q.dequeue(), Some(4));
        assert_eq!(q.dequeue(), Some(5));
        assert_eq!(q.dequeue(), Some(6));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn into_iter() {
        let mut q = Queue::new();
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);

        let mut iterator = q.into_iter();
        assert_eq!(iterator.next(), Some(4));
        assert_eq!(iterator.next(), Some(5));
        assert_eq!(iterator.next(), Some(6));
        assert_eq!(iterator.next(), None);
    }

    #[test]
    fn for_loop() {
        let mut q = Queue::new();
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);

        let mut v = vec![];
        for i in q {
            v.push(i);
        }
        assert_eq!(v, vec![4, 5, 6]);
    }

    #[test]
    fn iter() {
        let mut q = Queue::new();
        q.enqueue(4);
        q.enqueue(5);
        q.enqueue(6);

        let mut iterator = q.iter();
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), None);
    }
}
