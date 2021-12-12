//! # Max priority queue
//! Max priority queue implemented with a binary heap.
//! The largest key in a heap-sorted binary tree is found at the root.
use std::cmp::PartialOrd;

pub struct MaxPQ<T> {
    // the parent of the node in position `k` is in position `k/2`
    pq: Vec<T>, // heap-ordered complete binary tree
    n: usize,   // in pq[1..N] with pq[0] unused
}

impl<T: Default + Copy + PartialOrd> MaxPQ<T> {
    pub fn new(max_n: usize) -> Self {
        MaxPQ {
            pq: vec![T::default(); max_n + 1],
            n: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn max(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        Some(self.pq[1])
    }

    pub fn insert(&mut self, t: T) {
        self.n += 1;
        self.pq[self.n] = t;
        self.swim(self.n);
    }

    pub fn del_max(&mut self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        let max = self.pq[1];
        self.pq.swap(1, self.n);
        self.n -= 1;
        self.sink(1);
        Some(max)
    }

    // bottom-up reheapify, used in `insert`
    //
    // If the heap order is violated because a node’s key becomes
    // larger than that node’s parent’s key, then we can make progress toward
    // fixing the violation by exchanging the node with its parent.
    fn swim(&mut self, k: usize) {
        let mut index = k;
        while index > 1 && self.pq[index / 2] < self.pq[index] {
            self.pq.swap(index / 2, index);
            index /= 2;
        }
    }

    // top-down reheapify, used in `del_max`
    fn sink(&mut self, k: usize) {
        let mut index = k;
        while 2 * index <= self.n {
            let mut j = 2 * index;
            if j < self.n && self.pq[j] < self.pq[j + 1] {
                j += 1;
            }
            if self.pq[index] > self.pq[j] {
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
    fn pq_operations() {
        let mut pq = MaxPQ::new(15);
        // (P)
        pq.insert('P');
        //     (Q)
        //   /
        // (P)
        pq.insert('Q');
        //    (Q)
        //   /   \
        // (P)    (E)
        pq.insert('E');
        //     (P)
        //   /
        // (E)
        assert_eq!(pq.del_max(), Some('Q'));
        //    (X)
        //   /   \
        // (E)    (P)
        pq.insert('X');
        //       (X)
        //      /    \
        //    (E)     (P)
        //   /
        // (A)
        pq.insert('A');
        //       (X)
        //      /    \
        //    (M)     (P)
        //   /   \
        // (A)   (E)
        pq.insert('M');
        //       (P)
        //      /    \
        //    (M)     (E)
        //   /
        // (A)
        assert_eq!(pq.del_max(), Some('X'));
        //       (P)
        //      /    \
        //    (P)     (E)
        //   /   \
        // (A)   (M)
        pq.insert('P');
        //       (P)
        //      /    \
        //    (P)      (L)
        //   /   \    /
        // (A)   (M) (E)
        pq.insert('L');
        //       (P)
        //      /    \
        //    (P)      (L)
        //   /   \    /   \
        // (A)   (M) (E)   (E)
        pq.insert('E');
        //       (P)
        //      /    \
        //    (M)      (L)
        //   /   \    /
        // (A)   (E) (E)
        assert_eq!(pq.del_max(), Some('P'));
    }

    #[test]
    fn max() {
        let mut pq = MaxPQ::new(5);
        pq.insert(4);
        pq.insert(6);
        pq.insert(5);
        assert_eq!(pq.max(), Some(6));
        pq.del_max();
        assert_eq!(pq.max(), Some(5));
        pq.del_max();
        assert_eq!(pq.max(), Some(4));
        pq.del_max();
        assert_eq!(pq.max(), None)
    }
}
