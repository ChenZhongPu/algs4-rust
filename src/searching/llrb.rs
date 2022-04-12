//!# Red Black BST
//!
//! A set implemented using a left-leaning red-black BST.
//! This is the 2-3 version.

use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Color {
    Red,
    Black,
}

impl Color {
    fn flip(&self) -> Color {
        match self {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        }
    }
}

type Link<K> = Option<Box<Node<K>>>;
struct Node<K> {
    key: K,
    left: Link<K>,
    right: Link<K>,
    color: Color, // color of parent link
}

impl<K: Ord> Node<K> {
    fn new(k: K) -> Self {
        Node {
            key: k,
            left: None,
            right: None,
            color: Color::Red,
        }
    }

    fn is_red(x: &Link<K>) -> bool {
        match x {
            Some(node) => node.color == Color::Red,
            _ => false,
        }
    }

    // make a right-leaning link lean to the left
    //       E(:h)                    S
    //     /   \\                  //   \
    // (<E)     S (:x)      =>    E    (>S)
    //        /   \              /  \
    //   (>E,<S)  (>S)       (<E)  (>E,<S)
    fn rotate_left(mut self) -> Box<Node<K>> {
        match self.right {
            Some(mut x) => {
                assert_eq!(x.color, Color::Red);
                self.right = x.left.take();
                x.color = self.color;
                self.color = Color::Red;
                x.left = Some(Box::new(self));
                x
            }
            None => Box::new(self),
        }
    }
    // make a left-leaning link lean to the right
    //        S(:h)                    E
    //       //   \                  /  \\
    //     E(:x)   (>S)      =>    (<E)   S
    //   /   \                          /  \
    //  (<E)  (>E,<S)               (>E,<S) (>S)
    fn rotate_right(mut self) -> Box<Node<K>> {
        match self.left {
            Some(mut x) => {
                assert_eq!(x.color, Color::Red);
                self.left = x.right.take();
                x.color = self.color;
                self.color = Color::Red;
                x.right = Some(Box::new(self));
                x
            }
            None => Box::new(self),
        }
    }

    // flip the color of a node
    // it must have opposite color of its two children
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

pub struct LLRB<K: Ord> {
    root: Link<K>,
}

impl<K: Ord> LLRB<K> {
    pub fn new() -> Self {
        LLRB { root: None }
    }

    pub fn height(&self) -> i32 {
        fn get_height<K>(x: &Link<K>) -> i32 {
            match x {
                Some(node) => 1 + get_height(&node.left).max(get_height(&node.right)),
                None => -1,
            }
        }
        get_height(&self.root)
    }

    pub fn contains(&self, k: &K) -> bool {
        fn _contain<K: Ord>(x: &Link<K>, k: &K) -> bool {
            match x {
                Some(node) => match k.cmp(&node.key) {
                    Ordering::Equal => true,
                    Ordering::Less => _contain(&node.left, k),
                    Ordering::Greater => _contain(&node.right, k),
                },
                None => false,
            }
        }

        _contain(&self.root, k)
    }

    pub fn put(&mut self, k: K) {
        fn _put<K: Ord>(new_node: Box<Node<K>>, h: Link<K>) -> Link<K> {
            match h {
                Some(mut node) => {
                    match new_node.key.cmp(&node.key) {
                        Ordering::Less => node.left = _put(new_node, node.left),
                        Ordering::Equal => {
                            return Some(node);
                        }
                        Ordering::Greater => node.right = _put(new_node, node.right),
                    };

                    if Node::is_red(&node.right) && !Node::is_red(&node.left) {
                        node = node.rotate_left();
                    }

                    if Node::is_red(&node.left) {
                        // `node.left` is not None
                        let node_left = node.left.as_ref().unwrap();
                        if Node::is_red(&node_left.left) {
                            node = node.rotate_right();
                        }
                    }

                    if Node::is_red(&node.left) && Node::is_red(&node.right) {
                        node.flip_color();
                    }

                    Some(node)
                }
                None => Some(new_node),
            }
        }

        let new_node = Box::new(Node::new(k));

        self.root = _put(new_node, self.root.take());

        if let Some(ref mut root) = self.root {
            root.color = Color::Black;
        }

        self.check();
    }
}

// check integrity of LLRB
impl<K: Ord> LLRB<K> {
    fn check(&self) {
        if !self.is_bst() {
            panic!("Not in symmetric order");
        }

        if !self.is_balanced() {
            panic!("Not balanced");
        }

        if !self.is_2_3() {
            panic!("Not a 2-3 tree");
        }
    }

    fn is_bst(&self) -> bool {
        fn _is_bst<K: Ord>(x: &Link<K>, min: Option<&K>, max: Option<&K>) -> bool {
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

                    _is_bst(&node.left, min, Some(&node.key))
                        && _is_bst(&node.right, Some(&node.key), max)
                }
                None => true,
            }
        }

        _is_bst(&self.root, None, None)
    }

    fn is_balanced(&self) -> bool {
        // does every path from the root to a leaf have the given number of black links
        fn _is_balanced<K>(x: &Link<K>, black: i32) -> bool {
            match x {
                Some(node) => {
                    let mut _b = black;
                    if node.color == Color::Black {
                        _b -= 1;
                    }

                    _is_balanced(&node.left, _b) && _is_balanced(&node.right, _b)
                }
                None => black == 0,
            }
        }

        let mut black = 0;
        let mut current = &self.root;
        while let Some(node) = current {
            if node.color == Color::Black {
                black += 1;
            }
            current = &node.left;
        }

        _is_balanced(&self.root, black)
    }

    fn is_2_3(&self) -> bool {
        // Does the tree have no red right links, and at most one (left)
        // red links in a row on any path?
        fn _is_2_3<K: Ord>(x: &Link<K>) -> bool {
            match x {
                Some(node) => {
                    if Node::is_red(&node.right) {
                        return false;
                    }

                    if node.color == Color::Red && Node::is_red(&node.left) {
                        return false;
                    }

                    _is_2_3(&node.left) && _is_2_3(&node.right)
                }
                None => true,
            }
        }

        _is_2_3(&self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put() {
        let mut set = LLRB::new();
        set.put(1);
        set.put(5);
        set.put(3);
        set.put(2);
        set.put(8);
        set.put(6);

        assert!(set.contains(&5));

        assert!(!set.contains(&4));
    }

    #[test]
    fn height() {
        let mut set = LLRB::new();

        for i in 0..200 {
            set.put(i);
        }
        assert_eq!(set.height(), 7);
    }
}
