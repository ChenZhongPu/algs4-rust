//! # Symbol-table implementation with linear-probing hash table.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

const INIT_CAPACITY: usize = 4;

pub struct LinearProbingHashST<K, V> {
    n: usize, // number of key-value pairs
    m: usize, // size of linear probing table. m > n
    keys: Vec<Option<K>>,
    values: Vec<Option<V>>,
}

impl<K: Eq + Hash + Clone, V: Clone> LinearProbingHashST<K, V> {
    pub fn new(capacity: usize) -> Self {
        LinearProbingHashST {
            n: 0,
            m: capacity,
            keys: vec![None; capacity],
            values: vec![None; capacity],
        }
    }

    fn hash(&self, k: &K) -> usize {
        let mut s = DefaultHasher::new();
        k.hash(&mut s);
        (s.finish() as usize) % self.m
    }

    /// Returns the number of key-value pairs in this symbol table.
    pub fn size(&self) -> usize {
        self.n
    }

    /// Returns true if this symbol table is empty.
    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Returns the value associated with the specified key.
    pub fn get(&self, k: &K) -> Option<&V> {
        let mut i = self.hash(k);
        while let Some(ref key) = self.keys[i] {
            if key == k {
                return self.values[i].as_ref();
            }
            i = (i + 1) % self.m;
        }
        None
    }

    // resizes the hash table to the given capacity by re-hashing all of the keys
    fn resize(&mut self, capacity: usize) {
        let mut temp = LinearProbingHashST::new(capacity);
        for i in 0..self.m {
            if let Some(k) = self.keys[i].take() {
                temp.put(k, self.values[i].take().unwrap());
            }
        }
        *self = temp;
    }

    /// Returns true if this symbol table contains the specified key.
    pub fn contains(&self, k: &K) -> bool {
        self.get(k).is_some()
    }

    /// Inserts the specified key-value pair into the symbol table,
    /// overwriting the old value with the new value if the symbol table already contains the specified key.
    pub fn put(&mut self, k: K, v: V) {
        // double table size if 50% full
        if self.n >= self.m / 2 {
            self.resize(2 * self.m);
        }

        let mut i = self.hash(&k);
        while let Some(ref key) = self.keys[i] {
            if key == &k {
                // overwriting
                self.values[i] = Some(v);
                return;
            }
            i = (i + 1) % self.m;
        }
        // new entry
        self.keys[i] = Some(k);
        self.values[i] = Some(v);
        self.n += 1;
    }

    /// Removes the specified key and its associated value from this symbol table
    pub fn delete(&mut self, k: &K) {
        if !self.contains(k) {
            return;
        }

        // find position i of k
        let mut i = self.hash(k);
        while let Some(ref key) = self.keys[i] {
            if key != k {
                i = (i + 1) % self.m;
            } else {
                break;
            }
        }

        // delete key and associated value
        self.keys[i] = None;
        self.values[i] = None;

        // rehash all keys in the same cluster
        i = (i + 1) % self.m;
        while let (Some(key), Some(val)) = (self.keys[i].take(), self.values[i].take()) {
            self.n -= 1;
            self.put(key, val);
            i = (i + 1) % self.m;
        }
        self.n -= 1;

        // halves size of array if it's 12.5% full or less
        if self.n > 0 && self.n <= self.m / 8 {
            self.resize(self.m / 2);
        }
    }

    pub fn keys(&self) -> Iter<'_, K, V> {
        Iter::new(self)
    }
}

pub struct Iter<'a, K, V> {
    queue: Vec<&'a K>,
    _phantom: PhantomData<V>,
}

impl<'a, K: Eq + Hash + Clone, V: Clone> Iter<'a, K, V> {
    pub fn new(hash_st: &'a LinearProbingHashST<K, V>) -> Self {
        let mut queue = Vec::with_capacity(hash_st.n);
        for i in 0..hash_st.m {
            if let Some(ref key) = hash_st.keys[i] {
                queue.push(key);
            }
        }
        Iter {
            queue,
            _phantom: PhantomData {},
        }
    }
}

impl<'a, K: Eq + Hash + Clone, V: Clone> Iterator for Iter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }
}

impl<K: Eq + Hash + Clone, V: Clone> Default for LinearProbingHashST<K, V> {
    fn default() -> Self {
        LinearProbingHashST::new(INIT_CAPACITY)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get() {
        let mut st = LinearProbingHashST::new(10);
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));

        assert_eq!(st.get(&1), Some(&String::from("one")));
        assert_eq!(st.get(&2), Some(&String::from("two")));
        assert_eq!(st.get(&3), None);
    }

    #[test]
    fn put_replace() {
        let mut st = LinearProbingHashST::new(10);
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));
        st.put(3, String::from("three"));

        st.put(1, String::from("ONE"));
        st.put(2, String::from("TWO"));
        st.put(3, String::from("THREE"));

        assert_eq!(st.get(&1), Some(&String::from("ONE")));
        assert_eq!(st.get(&2), Some(&String::from("TWO")));
        assert_eq!(st.get(&3), Some(&String::from("THREE")));
    }

    #[test]
    fn delete() {
        let mut st = LinearProbingHashST::new(10);

        st.put(1, String::from("one"));
        st.put(2, String::from("two"));
        st.put(3, String::from("three"));
        st.put(4, String::from("four"));

        assert_eq!(st.size(), 4);

        assert!(st.contains(&1));
        st.delete(&1);
        assert_eq!(st.size(), 3);
        assert!(!st.contains(&1));

        assert!(st.contains(&3));
        st.delete(&3);
        assert_eq!(st.size(), 2);
        assert!(!st.contains(&3));
    }

    #[test]
    fn resize() {
        let mut st = LinearProbingHashST::default();
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));
        st.put(3, String::from("three"));
        st.put(4, String::from("four"));

        st.resize(8);

        assert_eq!(st.size(), 4);
        assert_eq!(st.get(&1), Some(&String::from("one")));
        assert_eq!(st.get(&2), Some(&String::from("two")));
        assert_eq!(st.get(&3), Some(&String::from("three")));
        assert_eq!(st.m, 8);
    }

    #[test]
    fn iterator() {
        let mut st = LinearProbingHashST::default();
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));
        st.put(3, String::from("three"));

        let mut v = vec![];
        for &k in st.keys() {
            v.push(k);
        }
        v.sort_unstable();
        assert_eq!(v, vec![1, 2, 3]);

        assert_eq!(st.size(), 3);
    }
}
