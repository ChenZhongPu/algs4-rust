//!# AVL tree is a self-balancing binary search tree.
//!
//! The difference between heights of left and right subtrees cannot be more than one for all nodes.
//! 
//! 

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
        Node { key: k, val: v, height: 1, left: None, right: None }
    }

    fn get_height(link: &Link<K, V>) -> usize {
        match link {
            None => 0,
            Some(node) => node.height
        }
    }

    fn update_height(node: &mut Box<Node<K, V>>) {
        node.height = Self::get_height(&node.left).max(Self::get_height(&node.right)) + 1;
    }
    
    fn get_balance_factor(link: &Link<K, V>) -> i8 {
        // left.height - right.height
        match link {
            None => 0,
            Some(node) => Node::balance_factor(node)
        }
    }

    fn balance_factor(node: &Box<Node<K, V>>) -> i8 {
        (Node::get_height(&node.left) as i64 - Node::get_height(&node.right) as i64) as i8
    }

    fn right_rotate(mut y: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if let Some(mut x) = y.left {
            y.left = x.right.take();
            Node::update_height(&mut y);
            x.right = Some(y);
            Node::update_height(&mut x);
            x
        } else {
            y // never reach here
        }
    }

    fn left_rotate(mut x: Box<Node<K, V>>) -> Box<Node<K, V>> {
        if let Some(mut y) = x.right {
            x.right = y.left.take();
            Node::update_height(&mut x);
            y.left = Some(x);
            Node::update_height(&mut y);
            y
        } else {
            x // never reach here
        }
    }
}


impl<K, V> Node<K, V> {

    fn min_key(x: &Box<Node<K, V>>) -> &K {
        if x.left.is_none() {
            &x.key
        } else {
            Self::min_key(x.left.as_ref().unwrap())
        }
    }

    fn max_key(x: &Box<Node<K, V>>) -> &K {
        if x.right.is_none() {
            &x.key
        } else {
            Self::max_key(x.right.as_ref().unwrap())
        }
    }
}

impl<K, V> Node<K, V> {
    // it has a bug: not update the height (current -> x)
    // fn extract_min(x: &mut Link<K, V>) -> Box<Node<K, V>> {
    //     let mut current = x;
    //     while current.as_ref().unwrap().left.is_some() {
    //         current = &mut current.as_mut().unwrap().left;
    //     }
    //     let node = current.take().unwrap();
    //     let left_most = Box::new(Node::new(node.key, node.val));
    //     *current = node.right;
    //     left_most
    // }

    fn extract_min_left(mut x: Box<Node<K, V>>, left: Box<Node<K, V>>) -> (Link<K, V>, Box<Node<K, V>>) {
        let (new_left, min) = Self::extract_min(left);
        x.left = new_left;
        Self::update_height(&mut x);
        (Some(x), min)
    }

    fn extract_min(mut x: Box<Node<K, V>>) -> (Link<K, V>, Box<Node<K, V>>) {
        match x.left.take() {
            Some(left) => Self::extract_min_left(x, left),
            None => (x.right.take(), x)
        }
    }
}

impl<K, V> Node<K, V> {
    fn re_balance(mut x: Box<Node<K, V>>) -> Box<Node<K, V>> {
        Node::update_height(&mut x);
        let bf = Node::balance_factor(&x);
        if bf > 1 && Node::get_balance_factor(&x.left) >= 0 {
            return Node::right_rotate(x);
        }
        if bf < -1 && Node::get_balance_factor(&x.right) <= 0 {
            return Node::left_rotate(x);
        }
        if bf > 1 && Node::get_balance_factor(&x.left) < 0 {
            x.left = Some(Node::left_rotate(x.left.take().unwrap()));
            return Node::right_rotate(x);
        }
        if bf < -1 && Node::get_balance_factor(&x.right) > 0 {
            x.right = Some(Node::right_rotate(x.right.take().unwrap()));
            return Node::left_rotate(x);
        }
        x
    }
}

pub struct AVL<K, V> {
    root: Link<K, V>
}

impl<K, V> AVL<K, V> {
    pub fn new() -> Self {
        AVL { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn height(&self) -> usize {
        Node::get_height(&self.root)
    }
}

impl<K: Ord, V> AVL<K, V> {

    pub fn min(&self) -> Option<&K> {
        self.root.as_ref().map(|node| Node::min_key(node))
    }

    pub fn max(&self) -> Option<&K> {
        self.root.as_ref().map(|node| Node::max_key(node))
    }
}

impl<K: Ord, V> AVL<K, V> {
    fn _get<'a, 'b>(x: &'a Link<K, V>, key: &'b K) -> Option<&'a V> {
        match x {
            None => None,
            Some(node) => match key.cmp(&node.key) {
                std::cmp::Ordering::Less => Self::_get(&node.left, key),
                std::cmp::Ordering::Equal => Some(&node.val),
                std::cmp::Ordering::Greater => Self::_get(&node.right, key),
            }
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        Self::_get(&self.root, key)
    }

    pub fn contains(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

impl<K: Ord, V> AVL<K, V> {
    fn _put(key: K, value: V, current: Link<K, V>) -> Link<K, V> {
        match current {
            None => return Some(Box::new(Node::new(key, value))),
            Some(mut x) => {
                match key.cmp(&x.key) {
                    std::cmp::Ordering::Less => x.left = AVL::_put(key, value, x.left.take()),
                    std::cmp::Ordering::Equal => x.val = value,
                    std::cmp::Ordering::Greater => x.right = AVL::_put(key, value, x.right.take()),
                }
                Some(Node::re_balance(x))
            }
        }
    }
    
    pub fn put(&mut self, key: K, value: V) {
        self.root = AVL::_put(key, value, self.root.take());
        self.check();
    }
}

impl<K: Ord, V> AVL<K, V> {
    fn _remove(key: &K, current: Link<K, V>) -> Link<K, V> {
        match current {
            Some(mut x) => {
                match key.cmp(&x.key){
                    std::cmp::Ordering::Less => x.left = AVL::_remove(key, x.left.take()),
                    std::cmp::Ordering::Equal => {
                        if x.left.is_none() {
                            return x.right;
                        }
                        if x.right.is_none() {
                            return x.left;
                        }
                        // let mut t = x;
                        // x = Node::extract_min(&mut t.right);
                        // x.right = t.right;
                        // x.left = t.left;
                        let t = x;
                        let (new_right, new_root) = Node::extract_min(t.right.unwrap());
                        x = new_root;
                        x.right = new_right;
                        x.left = t.left;
                    },
                    std::cmp::Ordering::Greater => x.right = AVL::_remove(key, x.right.take()),
                }
                Some(Node::re_balance(x))
            },
            None => None,
        }
    }

    pub fn remove(&mut self, key: &K) {
        self.root = AVL::_remove(key, self.root.take());
        self.check();
    }

}

impl<K: Ord, V> AVL<K, V> {

    fn check(&self) {
        if !self.is_bst() {
            panic!("Not in symmetric order");
        }
        if !self.is_balanced() {
            panic!("Not balanced");
        }
    }

    fn is_balanced(&self) -> bool {
        AVL::_is_balanced(&self.root)
    }

    fn _is_balanced(x: &Link<K, V>) -> bool {
        match x {
            Some(node) => {
                if Node::balance_factor(node).abs() > 1 {
                    return false;
                }
                AVL::_is_balanced(&node.left) && AVL::_is_balanced(&node.right)
            }
            None => true,
        }
    }

    fn is_bst(&self) -> bool {
        match &self.root {
            None => true,
            Some(node) => Self::_is_bst(&self.root, Node::min_key(node), Node::max_key(node))
        }
    }

    fn _is_bst(x: &Link<K, V>, min: &K, max: &K) -> bool {
        match x {
            Some(node) => {
                return &node.key >= min && &node.key <= max &&
                    Self::_is_bst(&node.left, min, &node.key) &&
                    Self::_is_bst(&node.right, &node.key, max)
            },
            None => true
        }
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
        st.put(5, String::from("FIVE"));

        assert_eq!(st.get(&5), Some(&String::from("FIVE")));
    }

    #[test]
    fn balance() {
        let mut st = AVL::new();
        for i in 0..200 {
            st.put(i, i.to_string());
        }
        assert_eq!(st.height(), 8);
    }

    #[test]
    fn delete() {
        let mut st = AVL::new();

        for i in 0..1000 {
            st.put(i, i.to_string());
        }
        assert_eq!(st.contains(&600), true);

        for i in (500..1000).step_by(10) {
            st.remove(&i);
        }
        assert_eq!(st.contains(&600), false);
    }
}