//! # Index min priority queue
//!
//! It is to associate a unique integer `index` with each item.
//! Given (i, t), keys[i] = t; inverse_pq[i] = j, and pq[j] = i.
use std::cmp::PartialOrd;

pub struct IndexMinPQ<T> {
    pq: Vec<usize>,         // binary heap using 1-based indexing
    inverse_pq: Vec<usize>, // inverse of pq: inverse[pq[i]] = pq[inverse[i]] = i
    n: usize,
    max_n: usize,
    keys: Vec<T>,
}

impl<T: Default + Copy + PartialOrd> IndexMinPQ<T> {
    pub fn new(max_n: usize) -> Self {
        IndexMinPQ {
            pq: vec![0; max_n + 1],
            inverse_pq: vec![0; max_n + 1],
            n: 0,
            max_n,
            keys: vec![T::default(); max_n + 1],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn contains(&self, i: usize) -> bool {
        assert!(i < self.max_n);
        self.inverse_pq[i] != 0
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn insert(&mut self, i: usize, t: T) {
        if self.contains(i) {
            panic!("index is already in the priority queue");
        }

        self.n += 1;
        self.inverse_pq[i] = self.n;
        self.pq[self.n] = i;
        self.keys[i] = t;
        self.swim(self.n);
    }

    pub fn min_index(&self) -> Option<usize> {
        if self.n == 0 {
            return None;
        }
        Some(self.pq[1])
    }

    pub fn min_key(&self) -> Option<T> {
        if self.n == 0 {
            return None;
        }
        Some(self.keys[self.pq[1]])
    }

    pub fn del_min(&mut self) -> Option<usize> {
        if self.n == 0 {
            return None;
        }

        let min = self.pq[1];
        self.exch(1, self.n);
        self.n -= 1;
        self.sink(1);
        assert_eq!(min, self.pq[self.n + 1]);
        self.inverse_pq[min] = 0;
        self.pq[self.n + 1] = 0;
        Some(min)
    }

    pub fn key_of(&self, i: usize) -> Option<T> {
        if !self.contains(i) {
            return None;
        }
        Some(self.keys[i])
    }

    /// Change the key associated with index `i` to the specified value
    pub fn change_key(&mut self, i: usize, t: T) {
        if !self.contains(i) {
            panic!("no such element");
        }
        self.keys[i] = t;
        // `swim` and `sink` can be exchanged
        self.swim(self.inverse_pq[i]);
        self.sink(self.inverse_pq[i]);
    }

    /// Decrease the key associated with index i to the specified value
    pub fn decrease_key(&mut self, i: usize, key: T) {
        if !self.contains(i) {
            panic!("no such element");
        }
        if self.keys[i] <= key {
            panic!("Calling decrease_key() with a key >= current key");
        }
        self.keys[i] = key;
        self.swim(self.inverse_pq[i]);
    }

    /// Increase the key associated with index i to the specified value
    pub fn increase_key(&mut self, i: usize, key: T) {
        if !self.contains(i) {
            panic!("no such element");
        }
        if self.keys[i] >= key {
            panic!("Calling increase() with a key <= current key");
        }
        self.keys[i] = key;
        self.sink(self.inverse_pq[i]);
    }

    fn greater(&self, i: usize, j: usize) -> bool {
        self.keys[self.pq[i]] > self.keys[self.pq[j]]
    }

    fn exch(&mut self, i: usize, j: usize) {
        self.pq.swap(i, j);
        self.inverse_pq[self.pq[i]] = i;
        self.inverse_pq[self.pq[j]] = j;
    }

    fn swim(&mut self, k: usize) {
        let mut index = k;
        while index > 1 && self.greater(index / 2, index) {
            self.exch(index, index / 2);
            index /= 2;
        }
    }

    fn sink(&mut self, k: usize) {
        let mut index = k;
        while 2 * index <= self.n {
            let mut j = 2 * index;
            if j < self.n && self.greater(j, j + 1) {
                j += 1;
            }
            if !self.greater(index, j) {
                break;
            }
            self.exch(index, j);
            index = j;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_del() {
        let v = [
            "it", "was", "the", "best", "of", "times", "it", "was", "the", "worst",
        ];

        let mut pq = IndexMinPQ::new(v.len());

        for (i, &item) in v.iter().enumerate() {
            pq.insert(i, item);
        }

        let mut index = vec![];
        while !pq.is_empty() {
            index.push(pq.del_min().unwrap());
        }
        // best it it of the the times was was worst
        assert_eq!(index, vec![3, 0, 6, 4, 8, 2, 5, 7, 1, 9]);
    }

    #[test]
    fn key_of() {
        let v = ["it", "was", "the", "best"];

        let mut pq = IndexMinPQ::new(10);

        for (i, &item) in v.iter().enumerate() {
            pq.insert(i, item);
        }

        assert_eq!(pq.key_of(0), Some("it"));
        assert_eq!(pq.key_of(1), Some("was"));
        assert_eq!(pq.key_of(2), Some("the"));
        assert_eq!(pq.key_of(3), Some("best"));
        assert_eq!(pq.key_of(4), None);
    }

    #[test]
    fn change_key() {
        let v = ["it", "was", "the", "best"];

        let mut pq = IndexMinPQ::new(v.len());

        for (i, &item) in v.iter().enumerate() {
            pq.insert(i, item);
        }
        assert_eq!(pq.min_index(), Some(3));
        pq.change_key(0, "apple");

        assert_eq!(pq.key_of(0), Some("apple"));
        assert_eq!(pq.min_index(), Some(0));
    }
}
