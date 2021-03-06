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
                    self.n -= 1;
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

pub struct IntoItemIter<K, V> {
    next: Link<K, V>,
}

impl<K, V> Iterator for IntoItemIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next;
            (node.key, node.val)
        })
    }
}

impl<K, V> SequentialSearchST<K, V> {
    pub fn into_items(self) -> IntoItemIter<K, V> {
        IntoItemIter { next: self.first }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

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
    fn put_replace() {
        let mut st = SequentialSearchST::new();
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
        let mut st = SequentialSearchST::new();
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
    fn iterator() {
        let mut st = SequentialSearchST::new();
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));
        st.put(3, String::from("three"));

        let mut v = vec![];
        for &k in st.keys() {
            v.push(k);
        }
        v.sort_unstable();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn items() {
        let mut st = SequentialSearchST::new();
        st.put(1, String::from("one"));
        st.put(2, String::from("two"));
        st.put(3, String::from("three"));

        let mut v1 = vec![];
        let mut v2 = vec![];
        for (k, v) in st.into_items() {
            v1.push(k);
            v2.push(v);
        }
        assert_eq!(v1, vec![3, 2, 1]);
        assert_eq!(
            v2,
            vec![
                String::from("three"),
                String::from("two"),
                String::from("one")
            ]
        );
    }
}
