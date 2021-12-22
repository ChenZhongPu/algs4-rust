//! # Binary search tree symbol table
//!
//! refactor `delete`

use std::cmp::Ord;
use std::cmp::Ordering;

type Link<K, V> = Option<Box<Node<K, V>>>;
#[derive(Debug)]
struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
    n: usize, // nodes in subtree rooted here
}

impl<K: Ord, V> Node<K, V> {
    fn new(k: K, v: V) -> Self {
        Node {
            key: k,
            val: v,
            left: None,
            right: None,
            n: 1,
        }
    }
}

pub struct BST<K, V> {
    root: Link<K, V>, // root of BST
}

impl<K: Ord, V> BST<K, V> {
    pub fn new() -> Self {
        BST { root: None }
    }

    fn _size(x: &Link<K, V>) -> usize {
        match x {
            Some(node) => node.n,
            None => 0,
        }
    }

    /// Returns the number of key-value pairs in this symbol table.
    pub fn size(&self) -> usize {
        Self::_size(&self.root)
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn _get<'a, 'b>(x: &'a Link<K, V>, k: &'b K) -> Option<&'a V> {
        if let Some(node) = x {
            match k.cmp(&node.key) {
                Ordering::Less => Self::_get(&node.left, k),
                Ordering::Greater => Self::_get(&node.right, k),
                Ordering::Equal => Some(&node.val),
            }
        } else {
            None
        }
    }

    /// Returns the value associated with the given key.
    pub fn get(&self, k: &K) -> Option<&V> {
        Self::_get(&self.root, k)
    }

    pub fn contains(&self, k: &K) -> bool {
        self.get(k).is_some()
    }

    fn _min(x: &Link<K, V>) -> Option<&K> {
        match x {
            Some(node) => match node.left {
                Some(_) => Self::_min(&node.left),
                _ => Some(&node.key),
            },
            _ => None,
        }
    }

    pub fn min(&self) -> Option<&K> {
        Self::_min(&self.root)
    }

    fn _max(x: &Link<K, V>) -> Option<&K> {
        match x {
            Some(node) => match node.right {
                Some(_) => Self::_max(&node.right),
                _ => Some(&node.key),
            },
            _ => None,
        }
    }

    pub fn max(&self) -> Option<&K> {
        Self::_max(&self.root)
    }
}

// put
impl<K: Ord, V> BST<K, V> {
    fn _put(new_node: Box<Node<K, V>>, x: &mut Link<K, V>) {
        if let Some(node) = x {
            match new_node.key.cmp(&node.key) {
                Ordering::Less => Self::_put(new_node, &mut node.left),
                Ordering::Greater => Self::_put(new_node, &mut node.right),
                Ordering::Equal => node.val = new_node.val,
            }
            node.n = 1 + Self::_size(&node.left) + Self::_size(&node.right);
        } else {
            *x = Some(new_node);
        }
    }

    pub fn put(&mut self, k: K, v: V) {
        let new_node = Box::new(Node::new(k, v));

        Self::_put(new_node, &mut self.root);

        assert!(self.check());
    }
}

// delete min and delete max
impl<K: Ord, V> BST<K, V> {
    pub fn delete_min(&mut self) {
        Self::_delete_min(&mut self.root);
        assert!(self.check());
    }

    fn _delete_min(x: &mut Link<K, V>) {
        if let Some(node) = x {
            match node.left {
                Some(_) => {
                    Self::_delete_min(&mut node.left);
                    node.n = Self::_size(&node.left) + Self::_size(&node.right) + 1;
                }
                _ => *x = node.right.take(),
            }
        }
    }

    fn extract_min(x: &mut Link<K, V>) -> Link<K, V> {
        if let Some(node) = x {
            match node.left {
                Some(_) => Self::extract_min(&mut node.left),
                _ => x.take(),
            }
        } else {
            None
        }
    }

    pub fn delete_max(&mut self) {
        Self::_delete_max(&mut self.root);
        assert!(self.check());
    }

    fn _delete_max(x: &mut Link<K, V>) {
        if let Some(node) = x {
            match node.right {
                Some(_) => {
                    Self::_delete_max(&mut node.right);
                    node.n = Self::_size(&node.left) + Self::_size(&node.right) + 1;
                }
                _ => *x = node.left.take(),
            }
        }
    }

    // https://stackoverflow.com/questions/66330144/
    fn _delete(x: &mut Link<K, V>, k: &K) {
        if let Some(node) = x {
            match k.cmp(&node.key) {
                Ordering::Less => {
                    Self::_delete(&mut node.left, k);
                    node.n = Self::_size(&node.left) + Self::_size(&node.right) + 1;
                }
                Ordering::Greater => {
                    Self::_delete(&mut node.right, k);
                    node.n = Self::_size(&node.left) + Self::_size(&node.right) + 1;
                }
                Ordering::Equal => match (node.left.as_ref(), node.right.as_ref()) {
                    (None, None) => *x = None,
                    (Some(_), None) => *x = node.left.take(),
                    (None, Some(_)) => *x = node.right.take(),
                    (Some(_), Some(_)) => {
                        *x = Self::extract_min(&mut node.right);
                    }
                },
            }
        }
    }

    pub fn delete(&mut self, k: &K) {
        Self::_delete(&mut self.root, k);
        assert!(self.check());
    }
}

// Check integrity of BST data structure.
impl<K: Ord, V> BST<K, V> {
    fn check(&self) -> bool {
        if !self.is_bst() {
            panic!("Not in symmetric order");
        }

        if !self.is_size_consistent() {
            panic!("Subtree counts not consistent");
        }

        self.is_bst() && self.is_size_consistent()
    }

    fn is_bst(&self) -> bool {
        Self::_is_bst(&self.root, None, None)
    }

    fn _is_bst(x: &Link<K, V>, min: Option<&K>, max: Option<&K>) -> bool {
        match x {
            Some(node) => {
                if let Some(min_key) = min {
                    if node.key <= *min_key {
                        return false;
                    }
                }

                if let Some(max_key) = max {
                    if node.key >= *max_key {
                        return false;
                    }
                }

                Self::_is_bst(&node.left, min, Some(&node.key))
                    && Self::_is_bst(&node.right, Some(&node.key), max)
            }
            _ => true,
        }
    }

    fn is_size_consistent(&self) -> bool {
        Self::_is_size_consistent(&self.root)
    }

    fn _is_size_consistent(x: &Link<K, V>) -> bool {
        match x {
            Some(node) => {
                if node.n != Self::_size(&node.left) + Self::_size(&node.right) + 1 {
                    return false;
                }
                Self::_is_size_consistent(&node.left) && Self::_is_size_consistent(&node.right)
            }
            _ => true,
        }
    }
}

impl<K: Ord, V> Default for BST<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_put() {
        let mut st = BST::new();
        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));

        assert_eq!(st.get(&5), Some(&String::from("five")));
        assert_eq!(st.size(), 4);

        st.put(8, String::from("eight"));
        st.put(6, String::from("six"));

        assert_eq!(st.get(&6), Some(&String::from("six")));
        assert_eq!(st.size(), 6);

        st.put(5, String::from("FIVE"));
        assert_eq!(st.get(&5), Some(&String::from("FIVE")));
        assert_eq!(st.size(), 6);
    }

    #[test]
    fn max_min() {
        let mut st = BST::new();
        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));
        st.put(8, String::from("eight"));
        st.put(6, String::from("six"));

        assert_eq!(st.max(), Some(&8));
        assert_eq!(st.min(), Some(&1));
    }

    #[test]
    fn delete1() {
        let mut st = BST::new();
        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));
        st.put(8, String::from("eight"));
        st.put(6, String::from("six"));

        st.delete_min();
        assert_eq!(st.size(), 5);
        assert_eq!(st.min(), Some(&2));

        st.delete_min();
        assert_eq!(st.size(), 4);
        assert_eq!(st.min(), Some(&3));

        st.delete_max();
        assert_eq!(st.size(), 3);
        assert_eq!(st.max(), Some(&6));
    }

    #[test]
    fn delete2() {
        let mut st = BST::new();
        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));
        st.put(8, String::from("eight"));
        st.put(6, String::from("six"));

        st.delete(&3);
        assert_eq!(st.contains(&3), false);
        st.delete(&8);
        assert_eq!(st.max(), Some(&6));

        assert_eq!(st.size(), 4);
    }
}
