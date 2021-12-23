//! # Binary search tree symbol table

use std::cmp::Ord;
use std::cmp::Ordering;
use std::panic;

type Link<K, V> = Option<Box<Node<K, V>>>;
struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
    n: usize, // nodes in subtree rooted here
}

pub struct BST<K, V> {
    root: Link<K, V>, // root of BST
}

impl<K: Ord, V> BST<K, V> {
    pub fn new() -> Self {
        BST { root: None }
    }
}

impl<K: Ord, V> BST<K, V> {
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

    pub fn contains(&self, k: &K) -> bool {
        self.get(k).is_some()
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

    /// Inserts the specified key-value pair into the symbol table,
    /// overwriting the old value with the new value
    /// if the symbol table already contains the specified key.
    pub fn put(&mut self, k: K, v: V) {
        let new_node = Box::new(Node {
            key: k,
            val: v,
            left: None,
            right: None,
            n: 1,
        });
        Self::_put(new_node, &mut self.root);

        self.check();
    }

    fn _put(new_node: Box<Node<K, V>>, current: &mut Link<K, V>) {
        if let Some(node) = current {
            match new_node.key.cmp(&node.key) {
                Ordering::Less => Self::_put(new_node, &mut node.left),
                Ordering::Greater => Self::_put(new_node, &mut node.right),
                Ordering::Equal => node.val = new_node.val, // replace
            }
            node.n = 1 + Self::_size(&node.left) + Self::_size(&node.right);
        } else {
            *current = Some(new_node);
        }
    }

    /// Removes the smallest key and associated value from the symbol table.
    pub fn delete_min(&mut self) {
        if self.is_empty() {
            panic!("Symbol table underflow");
        }
        Self::_delete_min(&mut self.root);
        assert!(self.check());
    }

    fn _delete_min(x: &mut Link<K, V>) {
        if let Some(node) = x {
            match node.left {
                None => *x = node.right.take(),
                Some(_) => {
                    Self::_delete_min(&mut node.left);
                    node.n = Self::_size(&node.left) + Self::_size(&node.right) + 1;
                }
            }
        }
    }

    /// Removes the largest key and associated value from the symbol table.
    pub fn delete_max(&mut self) {
        if self.is_empty() {
            panic!("Symbol table underflow");
        }
        Self::_delete_max(&mut self.root);
        assert!(self.check());
    }

    fn _delete_max(x: &mut Link<K, V>) {
        if let Some(node) = x {
            match node.right {
                None => *x = node.left.take(),
                Some(_) => {
                    Self::_delete_max(&mut node.right);
                    node.n = Self::_size(&node.left) + Self::_size(&node.right) + 1;
                }
            }
        }
    }

    pub fn delete(&mut self, target: &K) {
        if let Some(root) = self.root.take() {
            self.root = Self::_delete(root, target);
            assert!(self.check());
        }
    }

    // https://stackoverflow.com/questions/64043682/
    // `delete` will
    fn _delete(mut x: Box<Node<K, V>>, target: &K) -> Link<K, V> {
        if target < &x.key {
            if let Some(left) = x.left.take() {
                x.left = Self::_delete(left, target);
            }
            x.n = Self::_size(&x.left) + Self::_size(&x.right) + 1;
            return Some(x);
        }

        if target > &x.key {
            if let Some(right) = x.right.take() {
                x.right = Self::_delete(right, target);
            }
            x.n = Self::_size(&x.left) + Self::_size(&x.right) + 1;
            return Some(x);
        }

        assert!(target == &x.key);

        match (x.left.take(), x.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(mut left), Some(right)) => {
                if let Some(mut right_most) = Self::extract_max(&mut left) {
                    right_most.left = Some(left);
                    right_most.right = Some(right);
                    Some(right_most)
                } else {
                    left.right = Some(right);
                    Some(left)
                }
            }
        }
    }
}

impl<K: Ord, V> BST<K, V> {
    /// Returns the smallest key in the symbol table.
    pub fn min(&self) -> Option<&K> {
        Self::_min(&self.root)
    }

    fn _min(x: &Link<K, V>) -> Option<&K> {
        if let Some(node) = x {
            if node.left.is_none() {
                Some(&node.key)
            } else {
                Self::_min(&node.left)
            }
        } else {
            None
        }
    }

    fn extract_max(x: &mut Box<Node<K, V>>) -> Link<K, V> {
        match x.right {
            Some(ref mut right) => {
                if let Some(t) = Self::extract_max(right) {
                    Some(t)
                } else {
                    let mut r = x.right.take();
                    if let Some(ref mut y) = r {
                        x.right = std::mem::replace(&mut y.left, None);
                    }
                    r
                }
            }
            None => None,
        }
    }

    /// Returns the largest key in the symbol table.
    pub fn max(&self) -> Option<&K> {
        Self::_max(&self.root)
    }

    fn _max(x: &Link<K, V>) -> Option<&K> {
        if let Some(node) = x {
            if node.right.is_none() {
                Some(&node.key)
            } else {
                Self::_max(&node.right)
            }
        } else {
            None
        }
    }

    /// Returns the largest key in the symbol table
    /// less than or equal to `key`.
    pub fn floor(&self, key: &K) -> Option<&K> {
        Self::_floor(&self.root, key)
    }

    fn _floor<'a, 'b>(x: &'a Link<K, V>, key: &'b K) -> Option<&'a K> {
        if let Some(node) = x {
            match node.key.cmp(key) {
                Ordering::Equal => Some(&node.key),
                Ordering::Greater => Self::_floor(&node.left, key),
                Ordering::Less => match Self::_floor(&node.right, key) {
                    x_right @ Some(_) => x_right,
                    _ => Some(&node.key),
                },
            }
        } else {
            None
        }
    }

    /// Returns the smallest key in the symbol table greater than or equal to `key`.
    pub fn ceiling(&self, key: &K) -> Option<&K> {
        Self::_ceiling(&self.root, key)
    }

    fn _ceiling<'a, 'b>(x: &'a Link<K, V>, key: &'b K) -> Option<&'a K> {
        match x {
            Some(node) => match node.key.cmp(key) {
                Ordering::Equal => Some(&node.key),
                Ordering::Less => Self::_ceiling(&node.right, key),
                Ordering::Greater => match Self::_ceiling(&node.left, key) {
                    x_right @ Some(_) => x_right,
                    _ => Some(&node.key),
                },
            },
            _ => None,
        }
    }

    /// Return the key in the symbol table of a given `rank`.
    /// Note rank 0 is the smallest key.
    pub fn select(&self, rank: usize) -> Option<&K> {
        if rank >= self.size() {
            panic!("argument to select is invalid: {} ", rank);
        }
        Self::_select(&self.root, rank)
    }

    fn _select(x: &Link<K, V>, rank: usize) -> Option<&K> {
        if let Some(node) = x {
            let left_size = Self::_size(&node.left);
            match left_size.cmp(&rank) {
                Ordering::Equal => Some(&node.key),
                Ordering::Greater => Self::_select(&node.left, rank),
                Ordering::Less => Self::_select(&node.right, rank - left_size - 1),
            }
        } else {
            None
        }
    }

    /// Return the number of keys in the symbol table strictly less than `key`
    pub fn rank(&self, key: &K) -> usize {
        Self::_rank(&self.root, key)
    }

    fn _rank(x: &Link<K, V>, key: &K) -> usize {
        match x {
            Some(node) => match key.cmp(&node.key) {
                Ordering::Equal => Self::_size(&node.left),
                Ordering::Greater => 1 + Self::_size(&node.left) + Self::_rank(&node.right, key),
                Ordering::Less => Self::_rank(&node.left, key),
            },
            _ => 0,
        }
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

        assert_eq!(st.floor(&6), Some(&6));
        assert_eq!(st.floor(&7), Some(&6));
        assert_eq!(st.floor(&0), None);

        assert_eq!(st.ceiling(&7), Some(&8));
        assert_eq!(st.ceiling(&8), Some(&8));
        assert_eq!(st.ceiling(&9), None);

        assert_eq!(st.select(0), Some(&1));
        assert_eq!(st.select(2), Some(&3));

        assert_eq!(st.rank(&1), 0);
        assert_eq!(st.rank(&5), 3);
        assert_eq!(st.rank(&4), 3);
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
        assert!(!st.contains(&3));

        st.delete(&8);
        assert_eq!(st.max(), Some(&6));

        assert_eq!(st.size(), 4);
    }
}
