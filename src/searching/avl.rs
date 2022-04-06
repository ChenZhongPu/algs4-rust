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
                let t1_h = Node::height(&x.left);
                let t2_h = Node::height(&x.right);
                let t3_h = Node::height(&self.right);
                let y_h = t2_h.max(t3_h) + 1;
                let x_h = t1_h.max(y_h) + 1;

                self.left = x.right.take();
                self.height = y_h;
                x.right = Some(Box::new(self));
                x.height = x_h;
                x
            }
            _ => Box::new(self),
        }
    }

    fn rotate_left(mut self) -> Box<Node<K, V>> {
        match self.right {
            Some(mut y) => {
                let t1_h = Node::height(&self.left);
                let t2_h = Node::height(&y.left);
                let t3_h = Node::height(&y.right);
                let x_h = t1_h.max(t2_h) + 1;
                let y_h = x_h.max(t3_h) + 1;

                self.right = y.left.take();
                self.height = x_h;
                y.left = Some(Box::new(self));
                y.height = y_h;
                y
            }
            None => Box::new(self),
        }
    }

    pub fn rebalance(mut self) -> bool {
        match self.balance_factor() {
            -2 => {
                let right_node = self.right.as_mut().unwrap();
                if right_node.balance_factor() == 1 {
                    // self.right = Some(right_node.rotate_right());
                }
                true
            }
            2 => true,
            _ => false,
        }
    }
}
pub struct AVL<K, V> {
    root: Link<K, V>, // root of AVL
}

impl<K: Ord + Clone, V> AVL<K, V> {
    pub fn new() -> Self {
        AVL { root: None }
    }

    fn _put(new_node: Box<Node<K, V>>, current: Link<K, V>) -> Link<K, V> {
        match current {
            Some(mut node) => {
                let key = new_node.key.clone();
                // 1. Perform normal BST insertion
                match new_node.key.cmp(&node.key) {
                    std::cmp::Ordering::Less => node.left = Self::_put(new_node, node.left),
                    std::cmp::Ordering::Equal => node.val = new_node.val,
                    std::cmp::Ordering::Greater => node.right = Self::_put(new_node, node.right),
                }
                // 2. Update height

                // 3. Re-balance if needed

                Some(node)
            }
            None => Some(new_node),
        }
    }
}
