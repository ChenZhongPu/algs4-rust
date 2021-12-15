//! # Sequential search of symbol table
//! Symbol table implementation with sequential search in an unordered linked list of key-value pairs.
use std::cmp::Eq;

type Link<K, V> = Option<Box<Node<K, V>>>;
struct Node<K, V> {
    key: K,
    val: V,
    next: Link<K, V>,
}

pub struct SequentialSearchST<K, V> {
    first: Link<K, V>,
    n: usize,
}

impl<K: Eq, V> SequentialSearchST<K, V> {
    pub fn new() -> Self {
        SequentialSearchST { first: None, n: 0 }
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
        let mut head = self.first.as_deref();

        while let Some(node) = head {
            if node.key == *k {
                return Some(&node.val);
            }
            head = node.next.as_deref();
        }
        None
    }

    /// Inserts the specified key-value pair into the symbol table,
    /// overwriting the old value with the new value
    /// if the symbol table already contains the specified key.
    pub fn put(&mut self, k: K, v: V) {
        let mut head = self.first.as_deref_mut();
        while let Some(node) = head {
            if node.key == k {
                node.val = v;
                return;
            }
            head = node.next.as_deref_mut();
        }
        let new_node = Box::new(Node {
            key: k,
            val: v,
            next: self.first.take(),
        });
        self.first = Some(new_node);
        self.n += 1;
    }

    pub fn delete(&mut self, k: &K) {
        let mut current = &mut self.first;

        loop {
            match current {
                None => return,
                Some(node) if node.key == *k => {
                    *current = node.next.take();
                    return;
                }
                Some(node) => {
                    current = &mut node.next;
                }
            }
        }
    }
}

impl<K: Eq, V> Default for SequentialSearchST<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Iter<'a, K, V> {
    next: Option<&'a Node<K, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.key
        })
    }
}

impl<K, V> SequentialSearchST<K, V> {
    /// returns all keys in the symbol table as as iterator.
    /// note that the order is not important.
    pub fn keys(&self) -> Iter<'_, K, V> {
        Iter {
            next: self.first.as_deref(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_get() {
        let mut st = SequentialSearchST::new();
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));

        assert_eq!(st.get(&1), Some(&String::from("one")));
        assert_eq!(st.get(&2), Some(&String::from("two")));
        assert_eq!(st.get(&3), None);
    }

    #[test]
    fn delete() {
        let mut st = SequentialSearchST::new();
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));
        st.put(3, String::from("three"));
        st.put(4, String::from("four"));

        assert!(st.contains(&1));
        st.delete(&1);
        assert!(!st.contains(&1));

        assert!(st.contains(&3));
        st.delete(&3);
        assert!(!st.contains(&3));
    }

    #[test]
    fn iterator() {
        let mut st = SequentialSearchST::new();
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));
        st.put(3, String::from("three"));

        let mut v = vec![];
        for &k in st.keys() {
            v.push(k);
        }
        v.sort();
        assert_eq!(v, vec![1, 2, 3]);
    }
}
