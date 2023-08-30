//!# AVL tree is a self-balancing binary search tree.
//!
//! The difference between heights of left and right subtrees cannot be more than one for all nodes.
//!
//! Refer to https://francismurillo.github.io/2019-07-31-Understanding-Rust-Through-AVL-Trees/

type Link<K, V> = Option<Box<Node<K, V>>>;

struct Node<K, V> {
    key: K,
    val: V,
    height: usize,
    left: Link<K, V>,
    right: Link<K, V>,
}

impl<K, V> Node<K, V> {
    fn new(k: K, v: V) -> Self {
        Node {
            key: k,
            val: v,
            height: 1,
            left: None,
            right: None,
        }
    }

    fn height(node: &Link<K, V>) -> usize {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn balance_factor_link(node: &Link<K, V>) -> i8 {
        match node {
            Some(n) => {
                let left_h = Node::height(&n.left);
                let right_h = Node::height(&n.right);
                if left_h >= right_h {
                    (left_h - right_h) as i8
                } else {
                    -((right_h - left_h) as i8)
                }
            }
            None => 0,
        }
    }

    pub fn left_height(&self) -> usize {
        Node::height(&self.left)
    }

    pub fn right_height(&self) -> usize {
        Node::height(&self.right)
    }

    /// get balance factor: must be in range of [-2, 2].
    pub fn balance_factor(&self) -> i8 {
        let left_h = self.left_height();
        let right_h = self.right_height();

        if left_h >= right_h {
            (left_h - right_h) as i8
        } else {
            -((right_h - left_h) as i8)
        }
    }

    pub fn update_height(&mut self) {
        self.height = 1 + self.left_height().max(self.right_height());
    }
}

// T1, T2 and T3 are subtrees of the tree
// rooted with y (on the left side) or x (on
// the right side)
//      y                               x
//     / \     Right Rotation          /  \
//    x   T3   - - - - - - - >        T1   y
//   / \       < - - - - - - -            / \
//  T1  T2     Left Rotation            T2  T3
// Keys in both of the above trees follow the
// following order:
//  `keys(T1) < key(x) < keys(T2) < key(y) < keys(T3)`.
// So BST property is not violated anywhere.

impl<K: Ord, V> Node<K, V> {
    fn rotate_right(mut self) -> Box<Node<K, V>> {
        match self.left {
            Some(mut x) => {
                self.left = x.right.take();
                self.update_height();
                x.right = Some(Box::new(self));
                x.update_height();
                x
            }
            _ => Box::new(self),
        }
    }

    fn rotate_left(mut self) -> Box<Node<K, V>> {
        match self.right {
            Some(mut y) => {
                self.right = y.left.take();
                self.update_height();
                y.left = Some(Box::new(self));
                y.update_height();
                y
            }
            None => Box::new(self),
        }
    }

    // case 1: -2 (right right case)
    //   z                                y
    //  /  \                            /   \
    // T1   y     Left Rotate(z)       z      x
    //     /  \   - - - - - - - ->    / \    / \
    //    T2   x                     T1  T2 T3  T4
    //        / \
    //      T3  T4

    // case 2: -2 (right left case)
    //  z                            z                            x
    // / \                          / \                          /  \
    // T1  y   Right Rotate (y)    T1  x      Left Rotate(z)   z      y
    //   / \  - - - - - - - - ->     /  \   - - - - - - - ->  / \    / \
    //  x   T4                      T2   y                  T1  T2  T3  T4
    // / \                              /  \
    // T2   T3                           T3   T4

    // case 3: 2 (left left case)
    //  z                                      y
    // / \                                   /   \
    // y   T4      Right Rotate (z)          x      z
    // / \          - - - - - - - - ->      /  \    /  \
    // x   T3                               T1  T2  T3  T4
    // / \
    // T1   T2
    // case 4: 2 (left right case)
    //      z                               z                           x
    //     / \                            /   \                        /  \
    //     y   T4  Left Rotate (y)        x    T4  Right Rotate(z)    y      z
    //     / \      - - - - - - - - ->    /  \      - - - - - - - ->  / \    / \
    //     T1   x                          y    T3                   T1  T2 T3  T4
    //     / \                            / \
    //    T2   T3                      T1   T2
    pub fn re_balance(mut self) -> Box<Node<K, V>> {
        self.update_height();
        match self.balance_factor() {
            -2 => {
                // it must has a `right`
                if Node::balance_factor_link(&self.right) == 1 {
                    let y = self.right.take().unwrap();
                    self.right = Some(y.rotate_right());
                }
                self.rotate_left()
            }
            2 => {
                // it must has a `left`
                if Node::balance_factor_link(&self.left) == -1 {
                    let y = self.left.take().unwrap();
                    self.left = Some(y.rotate_left());
                }
                self.rotate_right()
            }
            _ => Box::new(self),
        }
    }
}
pub struct AVL<K, V> {
    root: Link<K, V>, // root of AVL
}

impl<K: Ord, V> AVL<K, V> {
    pub fn new() -> Self {
        AVL { root: None }
    }

    pub fn put(&mut self, k: K, v: V) {
        let new_node = Box::new(Node::new(k, v));
        self.root = AVL::_put(new_node, self.root.take());

        self.check();
    }

    fn _put(new_node: Box<Node<K, V>>, current: Link<K, V>) -> Link<K, V> {
        match current {
            Some(mut node) => {
                // 1. Perform normal BST insertion
                match new_node.key.cmp(&node.key) {
                    std::cmp::Ordering::Less => node.left = Self::_put(new_node, node.left),
                    std::cmp::Ordering::Equal => node.val = new_node.val,
                    std::cmp::Ordering::Greater => node.right = Self::_put(new_node, node.right),
                }
                // node.update_height();
                // 2. Update height, and 3. re-balance if needed
                node = Node::re_balance(*node);
                Some(node)
            }
            None => Some(new_node),
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        Self::_get(&self.root, k)
    }

    fn _get<'a>(x: &'a Link<K, V>, k: &K) -> Option<&'a V> {
        match x {
            Some(node) => match k.cmp(&node.key) {
                std::cmp::Ordering::Less => Self::_get(&node.left, k),
                std::cmp::Ordering::Equal => Some(&node.val),
                std::cmp::Ordering::Greater => Self::_get(&node.right, k),
            },
            None => None,
        }
    }
}

impl<K: Ord, V> AVL<K, V> {
    pub fn height(&self) -> usize {
        Node::height(&self.root)
    }
}

// Check integrity of AVL tree data structure.
impl<K: Ord, V> AVL<K, V> {
    fn check(&self) {
        if !self.is_bst() {
            panic!("Not in symmetric order");
        }
        if !self.is_balanced() {
            panic!("Not balanced");
        }
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

    fn is_balanced(&self) -> bool {
        AVL::_is_balanced(&self.root)
    }

    fn _is_balanced(x: &Link<K, V>) -> bool {
        match x {
            Some(node) => {
                if node.balance_factor().abs() > 1 {
                    return false;
                }
                AVL::_is_balanced(&node.left) && AVL::_is_balanced(&node.right)
            }
            None => true,
        }
    }
}

impl<K: Ord, V> Default for AVL<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_put() {
        let mut st = AVL::new();
        st.put(1, String::from("one"));
        st.put(5, String::from("five"));
        st.put(3, String::from("three"));
        st.put(2, String::from("two"));

        assert_eq!(st.get(&5), Some(&String::from("five")));

        st.put(6, String::from("six"));
        st.put(7, String::from("seven"));
        st.put(8, String::from("eight"));
        st.put(9, String::from("nine"));

        assert_eq!(st.get(&6), Some(&String::from("six")));

        assert_eq!(st.height(), 4);
    }

    #[test]
    fn balance() {
        let mut st = AVL::new();
        for i in 0..200 {
            st.put(i, i.to_string());
        }
        assert_eq!(st.height(), 8);
    }
}
