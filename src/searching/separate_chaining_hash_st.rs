//! # A symbol table implemented with a separate-chaining hash table.
use crate::searching::sequential_search_st::SequentialSearchST;
use std::marker::PhantomData;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub struct SeparateChainingHashST<K, V> {
    n: usize, // number of key-value pairs
    m: usize, // hash table size
    st: Vec<SequentialSearchST<K, V>>,
}

impl<K: Eq + Hash, V> SeparateChainingHashST<K, V> {
    pub fn new(m: usize) -> Self {
        let mut data: Vec<SequentialSearchST<K, V>> = Vec::with_capacity(m);
        for _ in 0..m {
            data.push(SequentialSearchST::new());
        }
        SeparateChainingHashST { n: 0, m, st: data }
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

    /// Returns the value associated with the specified key in this symbol table.
    pub fn get(&self, k: &K) -> Option<&V> {
        let i = self.hash(k);
        self.st[i].get(k)
    }

    /// Returns true if this symbol table contains the specified key.
    pub fn contains(&self, k: &K) -> bool {
        self.get(k).is_some()
    }

    fn resize(&mut self, chains: usize) {
        let mut tmp = SeparateChainingHashST::new(chains);

        while let Some(table) = self.st.pop() {
            for (k, v) in table.into_items() {
                tmp.put(k, v);
            }
        }
        *self = tmp;
    }

    /// Inserts the specified key-value pair into the symbol table, overwriting the old value with the new value if the symbol table already contains the specified key.
    pub fn put(&mut self, k: K, v: V) {
        // double table size if average length of list >= 10
        if self.n >= 10 * self.m {
            self.resize(2 * self.m);
        }
        let i = self.hash(&k);
        if !self.st[i].contains(&k) {
            self.n += 1;
        }
        self.st[i].put(k, v);
    }

    /// Removes the specified key and its associated value from this symbol table.
    pub fn delete(&mut self, k: &K) {
        let i = self.hash(k);
        if self.st[i].contains(k) {
            self.n -= 1;
        }

        self.st[i].delete(k);
        // halve table size if average length of list <= 2
        if self.m < 4 && self.n <= 2 * self.m {
            self.resize(self.m / 2);
        }
    }
}

pub struct Iter<'a, K, V> {
    queue: Vec<&'a K>,
    _phantom: PhantomData<V>,
}

impl<'a, K: Eq + Hash, V> Iter<'a, K, V> {
    pub fn new(hash_st: &'a SeparateChainingHashST<K, V>) -> Self {
        let mut queue = Vec::with_capacity(hash_st.n);
        for table in &hash_st.st {
            for key in table.keys() {
                queue.push(key);
            }
        }
        Iter {
            queue,
            _phantom: PhantomData {},
        }
    }
}

impl<'a, K: Eq + Hash, V> Iterator for Iter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop()
    }
}

impl<K: Eq + Hash, V> SeparateChainingHashST<K, V> {
    pub fn keys(&self) -> Iter<'_, K, V> {
        Iter::new(self)
    }
}

impl<K: Eq + Hash, V> Default for SeparateChainingHashST<K, V> {
    fn default() -> Self {
        SeparateChainingHashST::new(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get() {
        let mut st = SeparateChainingHashST::new(10);
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));

        assert_eq!(st.get(&1), Some(&String::from("one")));
        assert_eq!(st.get(&2), Some(&String::from("two")));
        assert_eq!(st.get(&3), None);
    }

    #[test]
    fn put_replace() {
        let mut st = SeparateChainingHashST::new(10);
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
        let mut st = SeparateChainingHashST::default();
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
        let mut st = SeparateChainingHashST::default();
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
        let mut st = SeparateChainingHashST::default();
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
