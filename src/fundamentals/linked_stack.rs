//! # Linked Stack
//!
//! A stack implemented with linked list
//!
//! https://rust-unofficial.github.io/too-many-lists/index.html

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    item: T,
    next: Link<T>,
}

pub struct LinkedStack<T> {
    first: Link<T>,
    n: usize,
}

impl<T> LinkedStack<T> {
    pub fn new() -> Self {
        LinkedStack { first: None, n: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn push(&mut self, t: T) {
        let new_node = Box::new(Node {
            item: t,
            next: self.first.take(),
        });
        self.first = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.first.take().map(|node| {
            self.first = node.next;
            node.item
        })
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.first.as_deref(),
        }
    }
}

impl<T> Default for LinkedStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for LinkedStack<T> {
    fn drop(&mut self) {
        let mut cur_link = self.first.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut s = LinkedStack::new();
        s.push(4);
        s.push(5);
        s.push(6);
        assert_eq!(s.pop(), Some(6));
        assert_eq!(s.pop(), Some(5));
        assert_eq!(s.pop(), Some(4));
        assert_eq!(s.pop(), None);
    }

    #[test]
    fn iter() {
        let mut s = LinkedStack::new();
        s.push(4);
        s.push(5);
        s.push(6);

        let mut iterator = s.iter();
        assert_eq!(iterator.next(), Some(&6));
        assert_eq!(iterator.next(), Some(&5));
        assert_eq!(iterator.next(), Some(&4));
        assert_eq!(iterator.next(), None);
    }
}
