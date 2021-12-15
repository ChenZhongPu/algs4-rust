//! # Binary search in an ordered array
//! Symbol table implementation with binary search in an ordered array.

use std::cmp::Ord;
use std::cmp::Ordering;

pub struct BinarySearchST<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
    n: usize, // can be omitted, because `keys.len()` is `n`.
}

impl<K: Ord, V> BinarySearchST<K, V> {
    pub fn new() -> Self {
        BinarySearchST {
            keys: Vec::new(),
            values: Vec::new(),
            n: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    pub fn contains(&self, k: &K) -> bool {
        self.get(k).is_some()
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        if self.is_empty() {
            return None;
        }

        let i = self.rank(k);

        if i < self.n && self.keys[i] == *k {
            return Some(&self.values[i]);
        }

        None
    }

    /// returns the number of keys in the symbol table strictly less than `k`
    pub fn rank(&self, k: &K) -> usize {
        let mut lo = 0;
        let mut hi = self.n as i32 - 1;
        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            match k.cmp(&self.keys[mid as usize]) {
                Ordering::Less => hi = mid - 1,
                Ordering::Greater => lo = mid + 1,
                Ordering::Equal => return mid as usize,
            }
        }
        lo as usize
    }

    pub fn put(&mut self, k: K, v: V) {
        let i = self.rank(&k);

        // key is already in table
        if i < self.n && self.keys[i] == k {
            self.values[i] = v;
            return;
        }

        // insert new key-value pair
        self.keys.insert(i, k);
        self.values.insert(i, v);

        self.n += 1;

        assert!(self.is_sorted());
    }

    pub fn delete(&mut self, k: &K) {
        if self.is_empty() {
            return;
        }

        let i = self.rank(k);

        // key not in table
        if i == self.n || self.keys[i] != *k {
            return;
        }

        self.keys.remove(i);
        self.values.remove(i);

        self.n -= 1;

        assert!(self.is_sorted());
    }

    // check internal invariants
    fn is_sorted(&self) -> bool {
        for i in 1..self.size() {
            if self.keys[i] < self.keys[i - 1] {
                return false;
            }
        }
        true
    }
}

impl<K: Ord, V> BinarySearchST<K, V> {
    pub fn min(&self) -> Option<&K> {
        // self.keys.get(0)
        self.keys.first()
    }

    pub fn max(&self) -> Option<&K> {
        // self.keys.get(n - 1)
        self.keys().last()
    }

    /// Return the kth smallest key in this symbol table.
    /// smallest = 0th
    pub fn select(&self, k: usize) -> Option<&K> {
        if k >= self.size() {
            return None;
        }
        Some(&self.keys[k])
    }

    /// Returns the largest key in this symbol table
    /// less than or equal to `k`.
    pub fn floor(&self, k: &K) -> Option<&K> {
        let i = self.rank(k);

        if i < self.n && self.keys[i] == *k {
            return Some(&self.keys[i]);
        }

        if i == 0 {
            None
        } else {
            Some(&self.keys[i - 1])
        }
    }

    /// Returns the smallest key in this symbol table
    /// greater than or equal to `k`.
    pub fn ceiling(&self, k: &K) -> Option<&K> {
        let i = self.rank(k);

        if i == self.n {
            None
        } else {
            Some(&self.keys[i])
        }
    }

    /// Returns the number of keys in this symbol table
    /// between `lo` (inclusive) and `hi` (inclusive).
    pub fn range_size(&self, lo: &K, hi: &K) -> usize {
        if lo > hi {
            return 0;
        }

        if self.contains(hi) {
            self.rank(hi) - self.rank(lo) + 1
        } else {
            self.rank(hi) - self.rank(lo)
        }
    }
}

impl<K: Ord, V> Default for BinarySearchST<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'a, K> {
    data: &'a Vec<K>,
    index: usize,
    end: usize,
}

impl<'a, K> Iterator for Iter<'a, K> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index <= self.end {
            let item = &self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<K: Ord, V> BinarySearchST<K, V> {
    pub fn keys(&self) -> Iter<'_, K> {
        Iter {
            data: &self.keys,
            index: 0,
            end: self.size() - 1,
        }
    }

    pub fn range_keys(&self, lo: &K, hi: &K) -> Iter<'_, K> {
        assert!(lo <= hi);
        let end = if self.contains(hi) {
            self.rank(hi)
        } else {
            self.rank(hi) - 1
        };
        Iter {
            data: &self.keys,
            index: self.rank(lo),
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get_delete() {
        let mut st = BinarySearchST::new();

        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));
        st.put(8, String::from("eight"));
        st.put(6, String::from("six"));

        assert_eq!(st.get(&3), Some(&String::from("three")));
        assert_eq!(st.get(&2), Some(&String::from("two")));

        st.put(5, String::from("FIVE"));
        assert_eq!(st.get(&5), Some(&String::from("FIVE")));

        st.delete(&5);
        assert_eq!(st.get(&5), None);

        assert_eq!(st.get(&4), None);
    }

    #[test]
    fn min_max() {
        let mut st = BinarySearchST::new();

        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));
        st.put(8, String::from("eight"));
        st.put(6, String::from("six"));

        assert_eq!(st.min(), Some(&1));
        assert_eq!(st.max(), Some(&8));
    }

    #[test]
    fn select_floor_ceil() {
        let mut st = BinarySearchST::new();

        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));
        st.put(8, String::from("eight"));
        st.put(6, String::from("six"));

        assert_eq!(st.select(1), Some(&2));
        assert_eq!(st.select(3), Some(&5));

        assert_eq!(st.floor(&6), Some(&6));
        assert_eq!(st.floor(&7), Some(&6));

        assert_eq!(st.ceiling(&7), Some(&8));
        assert_eq!(st.ceiling(&8), Some(&8));
        assert_eq!(st.ceiling(&9), None);

        assert_eq!(st.range_size(&2, &5), 3);
        assert_eq!(st.range_size(&9, &11), 0);
    }

    #[test]
    fn keys() {
        let mut st = BinarySearchST::new();

        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));
        st.put(8, String::from("eight"));
        st.put(6, String::from("six"));

        let mut v = vec![];
        for k in st.keys() {
            v.push(*k);
        }

        assert_eq!(v, vec![1, 2, 3, 5, 6, 8]);

        v = vec![];
        for k in st.range_keys(&3, &7) {
            v.push(*k);
        }

        assert_eq!(v, vec![3, 5, 6]);
    }
}
