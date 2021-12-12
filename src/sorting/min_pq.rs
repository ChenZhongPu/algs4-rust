//! # Min priority queue
//! Min priority queue implemented with a binary heap.
//! The smallest key in a heap-sorted binary tree is found at the root.
use std::cmp::PartialOrd;

pub struct MinPQ<T> {
    pq: Vec<T>,
    n: usize,
}

impl<T: Default + Copy + PartialOrd> MinPQ<T> {
    pub fn new(max_n: usize) -> Self {
        MinPQ {
            pq: vec![T::default(); max_n + 1],
            n: 0,
        }
    }

    /// resizing
    pub fn empty() -> Self {
        MinPQ::new(1)
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn min(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.pq[1])
    }

    pub fn insert(&mut self, t: T) {
        if self.n == self.pq.len() - 1 {
            self.pq.resize(2 * self.pq.len(), T::default());
        }
        self.n += 1;
        self.pq[self.n] = t;
        self.swim(self.n);
    }

    pub fn del_min(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let min = self.pq[1];
        self.pq.swap(1, self.n);
        self.n -= 1;
        self.sink(1);
        if self.n > 0 && self.n == self.pq.len() / 4 {
            self.pq.resize(self.pq.len() / 2, T::default());
        }
        Some(min)
    }

    fn swim(&mut self, k: usize) {
        let mut index = k;
        while index > 1 && self.pq[index] < self.pq[index / 2] {
            self.pq.swap(index / 2, index);
            index /= 2;
        }
    }

    fn sink(&mut self, k: usize) {
        let mut index = k;
        while 2 * index <= self.n {
            let mut j = 2 * index;
            if j < self.n && self.pq[j + 1] < self.pq[j] {
                j += 1;
            }
            if self.pq[index] < self.pq[j] {
                break;
            }
            self.pq.swap(index, j);
            index = j;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pg_operations() {
        let mut pq = MinPQ::empty();
        // (P)
        pq.insert('P');
        //     (P)
        //   /
        // (Q)
        pq.insert('Q');
        //    (E)
        //   /   \
        // (Q)    (P)
        pq.insert('E');
        //     (P)
        //   /
        // (Q)
        assert_eq!(pq.del_min(), Some('E'));
        //    (P)
        //   /   \
        // (Q)    (X)
        pq.insert('X');
        //       (A)
        //      /    \
        //    (P)     (X)
        //   /
        // (Q)
        pq.insert('A');
        //       (A)
        //      /    \
        //    (M)     (X)
        //   /  \
        // (Q)  (P)
        pq.insert('M');
        //       (M)
        //      /    \
        //    (P)     (X)
        //   /
        // (Q)
        assert_eq!(pq.del_min(), Some('A'));
    }

    #[test]
    fn min() {
        let mut pq = MinPQ::new(5);
        pq.insert(4);
        pq.insert(6);
        pq.insert(5);
        assert_eq!(pq.min(), Some(4));
        pq.del_min();
        assert_eq!(pq.min(), Some(5));
        pq.del_min();
        assert_eq!(pq.min(), Some(6));
        pq.del_min();
        assert_eq!(pq.min(), None);
    }
}
