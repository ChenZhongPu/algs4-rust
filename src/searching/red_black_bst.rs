//! # Red Black BST
//!
//! A symbol table implemented using a left-leaning red-black BST.
//! This is the 2-3 version.

use std::cmp::Ord;
use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

impl Color {
    fn flip(&self) -> Color {
        if *self == Color::Red {
            Color::Black
        } else {
            Color::Red
        }
    }
}

type Link<K, V> = Option<Box<Node<K, V>>>;
struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
    color: Color, // color of parent link
    n: usize,     // nodes in subtree rooted here
}

impl<K: Ord, V> Node<K, V> {
    fn new(k: K, v: V) -> Self {
        Node {
            key: k,
            val: v,
            left: None,
            right: None,
            color: Color::Red, // when inserted, the default color is `RED`
            n: 1,
        }
    }
    // make a right-leaning link lean to the left
    //       E(:h)                    S
    //     /   \\                  //   \
    // (<E)     S (:x)      =>    E    (>S)
    //        /   \              /  \
    //   (>E,<S)  (>S)       (<E)  (>E,<S)

    // x = h.right;
    // h.right = x.left;
    // x.left = h;
    // x.color = h.color;
    // h.color = RED;
    // x.n = h.n;
    // h.n = 1 + size(h.left) + size(h.right);
    // return x;
    fn rotate_left(mut self) -> Box<Node<K, V>> {
        match self.right {
            Some(mut x) => {
                assert_eq!(x.color, Color::Red);
                self.right = x.left.take();
                x.color = self.color;
                self.color = Color::Red;
                x.n = self.n;
                self.n = 1 + RedBlackBST::_size(&self.left) + RedBlackBST::_size(&self.right);
                x.left = Some(Box::new(self));
                x
            }
            _ => Box::new(self),
        }
    }
    // make a left-leaning link lean to the right
    //        S(:h)                    E
    //       //   \                  /  \\
    //     E(:x)   (>S)      =>    (<E)   S
    //   /   \                          /  \
    //  (<E)  (>E,<S)               (>E,<S) (>S)
    fn rotate_right(mut self) -> Box<Node<K, V>> {
        match self.left {
            Some(mut x) => {
                assert_eq!(x.color, Color::Red);
                self.left = x.right.take();
                x.color = self.color;
                self.color = Color::Red;
                x.n = self.n;
                self.n = 1 + RedBlackBST::_size(&self.left) + RedBlackBST::_size(&self.right);
                x.right = Some(Box::new(self));
                x
            }
            _ => Box::new(self),
        }
    }

    // flip the colors of a node and its two children
    // h must have opposite color of its two children
    fn flip_color(&mut self) {
        self.color = self.color.flip();
        if let Some(ref mut left) = self.left {
            left.color = left.color.flip();
        }
        if let Some(ref mut right) = self.right {
            right.color = right.color.flip();
        }
    }
}

// TODO! delete
// https://stackoverflow.com/questions/15455042/

pub struct RedBlackBST<K, V> {
    root: Link<K, V>,
}

impl<K: Ord, V> RedBlackBST<K, V> {
    pub fn new() -> Self {
        RedBlackBST { root: None }
    }

    fn is_red(x: &Link<K, V>) -> bool {
        match x {
            Some(node) => node.color == Color::Red,
            _ => false, // `None` is black by default
        }
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

    /// Is this symbol table empty?
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn _get<'a>(x: &'a Link<K, V>, k: &K) -> Option<&'a V> {
        match x {
            Some(node) => match k.cmp(&node.key) {
                Ordering::Equal => Some(&node.val),
                Ordering::Less => Self::_get(&node.left, k),
                Ordering::Greater => Self::_get(&node.right, k),
            },
            _ => None,
        }
    }

    /// Returns the value associated with the given key.
    pub fn get(&self, k: &K) -> Option<&V> {
        Self::_get(&self.root, k)
    }

    /// Does this symbol table contain the given key?
    pub fn contains(&self, k: &K) -> bool {
        self.get(k).is_none()
    }

    fn _height(x: &Link<K, V>) -> i32 {
        match x {
            Some(node) => 1 + Self::_height(&node.left).max(Self::_height(&node.right)),
            _ => -1,
        }
    }

    // Returns the height of the BST (for debugging).
    // Note a 1-node tree has height 0
    pub fn height(&self) -> i32 {
        Self::_height(&self.root)
    }
}

// put
impl<K: Ord, V> RedBlackBST<K, V> {
    fn _put(new_node: Box<Node<K, V>>, h: Link<K, V>) -> Link<K, V> {
        match h {
            Some(mut node) => {
                match new_node.key.cmp(&node.key) {
                    Ordering::Less => node.left = Self::_put(new_node, node.left),
                    Ordering::Greater => node.right = Self::_put(new_node, node.right),
                    Ordering::Equal => node.val = new_node.val,
                }

                if Self::is_red(&node.right) && !Self::is_red(&node.left) {
                    node = node.rotate_left();
                }
                if Self::is_red(&node.left) {
                    if let Some(ref node_left) = node.left {
                        if Self::is_red(&node_left.left) {
                            node = node.rotate_right();
                        }
                    }
                }
                if Self::is_red(&node.left) && Self::is_red(&node.right) {
                    node.flip_color();
                }
                node.n = 1 + Self::_size(&node.left) + Self::_size(&node.right);
                Some(node)
            }
            _ => Some(new_node),
        }
    }

    pub fn put(&mut self, k: K, v: V) {
        let new_node = Box::new(Node::new(k, v));
        self.root = Self::_put(new_node, self.root.take());
        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }
        assert!(self.check());
    }
}

// Ordered symbol table methods.
impl<K: Ord, V> RedBlackBST<K, V> {
    fn _min(x: &Link<K, V>) -> Option<&K> {
        match x {
            Some(node) => match node.left {
                Some(_) => Self::_min(&node.left),
                _ => Some(&node.key),
            },
            _ => None,
        }
    }

    /// Returns the smallest key in the symbol table.
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

    /// Returns the largest key in the symbol table.
    pub fn max(&self) -> Option<&K> {
        Self::_max(&self.root)
    }

    fn _floor<'a>(x: &'a Link<K, V>, k: &K) -> Option<&'a K> {
        match x {
            Some(node) => match k.cmp(&node.key) {
                Ordering::Equal => Some(&node.key),
                Ordering::Less => Self::_floor(&node.left, k),
                Ordering::Greater => match Self::_floor(&node.right, k) {
                    x_right @ Some(_) => x_right,
                    _ => Some(&node.key),
                },
            },
            _ => None,
        }
    }

    /// Returns the largest key in the symbol table less than or equal to `key`.
    pub fn floor(&self, k: &K) -> Option<&K> {
        Self::_floor(&self.root, k)
    }

    fn _ceiling<'a>(x: &'a Link<K, V>, k: &K) -> Option<&'a K> {
        match x {
            Some(node) => match k.cmp(&node.key) {
                Ordering::Equal => Some(&node.key),
                Ordering::Greater => Self::_ceiling(&node.right, k),
                Ordering::Less => match Self::_ceiling(&node.left, k) {
                    x_left @ Some(_) => x_left,
                    _ => Some(&node.key),
                },
            },
            _ => None,
        }
    }

    /// Returns the smallest key in the symbol table greater than or equal to `key`
    pub fn ceiling(&self, k: &K) -> Option<&K> {
        Self::_ceiling(&self.root, k)
    }

    fn _select(x: &Link<K, V>, rank: usize) -> Option<&K> {
        match x {
            Some(node) => {
                let left_size = Self::_size(&node.left);
                match left_size.cmp(&rank) {
                    Ordering::Equal => Some(&node.key),
                    Ordering::Greater => Self::_select(&node.left, rank),
                    Ordering::Less => Self::_select(&node.right, rank - left_size - 1),
                }
            }
            _ => None,
        }
    }

    /// Return the key in the symbol table of a given `rank`.
    /// Note rank 0 is the smallest key.
    pub fn select(&self, rank: usize) -> Option<&K> {
        if rank >= self.size() {
            return None;
        }

        Self::_select(&self.root, rank)
    }

    fn _rank(x: &Link<K, V>, k: &K) -> usize {
        match x {
            Some(node) => {
                let left_size = Self::_size(&node.left);
                match k.cmp(&node.key) {
                    Ordering::Equal => left_size,
                    Ordering::Less => Self::_rank(&node.left, k),
                    Ordering::Greater => 1 + left_size + Self::_rank(&node.right, k),
                }
            }
            _ => 0,
        }
    }

    /// Return the number of keys in the symbol table strictly less than `key`.
    pub fn rank(&self, k: &K) -> usize {
        Self::_rank(&self.root, k)
    }
}

impl<K: Ord, V> Default for RedBlackBST<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

// Check integrity of red-black tree data structure.
impl<K: Ord, V> RedBlackBST<K, V> {
    fn check(&self) -> bool {
        if !self.is_bst() {
            panic!("Not in symmetric order");
        }
        if !self.is_size_consistent() {
            panic!("Subtree counts not consistent");
        }
        if !self.is_balanced() {
            panic!("Not balanced")
        }
        if !self.is_2_3() {
            panic!("Not a 2-3 tree")
        }

        self.is_bst() && self.is_size_consistent() && self.is_balanced() && self.is_2_3()
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

    fn is_balanced(&self) -> bool {
        let mut black = 0;
        let mut current = &self.root;
        while let Some(node) = current {
            if node.color == Color::Black {
                black += 1;
            }
            current = &node.left;
        }
        Self::_is_balanced(&self.root, black)
    }

    // does every path from the root to a leaf have the given number of black links?
    fn _is_balanced(x: &Link<K, V>, black: i32) -> bool {
        match x {
            Some(node) => {
                let mut _b = black;
                if node.color == Color::Black {
                    _b -= 1;
                }
                Self::_is_balanced(&node.left, _b) && Self::_is_balanced(&node.right, _b)
            }
            _ => black == 0,
        }
    }

    fn is_2_3(&self) -> bool {
        Self::_is_2_3(&self.root)
    }
    // Does the tree have no red right links, and at most one (left)
    // red links in a row on any path?
    fn _is_2_3(x: &Link<K, V>) -> bool {
        match x {
            Some(node) => {
                if Self::is_red(&node.right) {
                    return false;
                }
                if node.color == Color::Red && Self::is_red(&node.left) {
                    return false;
                }
                Self::_is_2_3(&node.left) && Self::_is_2_3(&node.right)
            }
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_put() {
        let mut st = RedBlackBST::new();
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
    fn height() {
        // val does not make sense in this test.
        let mut st = RedBlackBST::new();
        st.put('A', 1);
        st.put('C', 2);
        st.put('E', 3);
        st.put('H', 4);
        st.put('L', 5);
        st.put('M', 6);
        st.put('P', 7);
        st.put('R', 8);
        st.put('S', 9);
        st.put('X', 10);
        assert_eq!(st.height(), 3);

        let mut st = RedBlackBST::new();
        st.put('S', 1);
        st.put('E', 2);
        st.put('A', 3);
        st.put('R', 4);
        st.put('C', 5);
        st.put('H', 6);
        st.put('X', 7);
        st.put('M', 8);
        st.put('P', 9);
        st.put('L', 10);
        assert_eq!(st.height(), 3);
    }

    #[test]
    fn min_max() {
        let mut st = RedBlackBST::new();
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
}
