//! # Determining depth-first search ordering of the vertices in a digraph. Including preorder, postorder and reverse postorder.
//!
//! This implementation uses depth-first search.
//! Note the results rely on the order of adj.

use super::{digraph::Digraph, weighted_digraph::EdgeWeightedDiagraph};
pub struct DepthFirstOrder {
    marked: Vec<bool>,
    // since only `enqueue` is required, we can use Vec
    pre: Vec<usize>, // queue: vertices in preorder
    post: Vec<usize>, // queue: vertices in postorder
                     // reverse postorder (a stack) can be obtained from post
}

impl DepthFirstOrder {
    pub fn new(g: &Digraph) -> Self {
        let mut dfs_order = DepthFirstOrder {
            marked: vec![false; g.v()],
            pre: vec![],
            post: vec![],
        };

        for v in 0..g.v() {
            if !dfs_order.marked[v] {
                dfs_order.dfs(g, v);
            }
        }

        dfs_order
    }

    pub fn from_weighted_diagraph(g: &EdgeWeightedDiagraph) -> Self {
        let mut dfs_order = DepthFirstOrder {
            marked: vec![false; g.v()],
            pre: vec![],
            post: vec![],
        };
        for v in 0..g.v() {
            if !dfs_order.marked[v] {
                dfs_order.dfs_weighted_digraph(g, v);
            }
        }

        dfs_order
    }

    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.pre.push(v);
        self.marked[v] = true;
        for w in g.adj(v).clone() {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
        self.post.push(v);
    }

    fn dfs_weighted_digraph(&mut self, g: &EdgeWeightedDiagraph, v: usize) {
        self.pre.push(v);
        self.marked[v] = true;
        for e in g.adj(v) {
            let w = e.to();
            if !self.marked[w] {
                self.dfs_weighted_digraph(g, w);
            }
        }
        self.post.push(v);
    }

    /// Returns the vertices in preorder.
    pub fn pre(&self) -> std::vec::IntoIter<usize> {
        self.pre.clone().into_iter()
    }

    /// Returns the vertices in postorder.
    pub fn post(&self) -> std::vec::IntoIter<usize> {
        self.post.clone().into_iter()
    }

    /// Returns the vertices in reverse postorder.
    pub fn rev_post(&self) -> RevPostIter {
        RevPostIter::new(self)
    }

    /// Returns the preorder number of v
    pub fn pre_of(&self, v: usize) -> usize {
        self.pre[v]
    }

    /// Returns the postorder number of v
    pub fn post_of(&self, v: usize) -> usize {
        self.post[v]
    }

    /// Returns the reverse postorder number of v
    pub fn rev_post_of(&self, v: usize) -> usize {
        self.post.len() - self.post_of(v) - 1
    }
}

pub struct RevPostIter {
    stack: Vec<usize>,
}

impl RevPostIter {
    pub fn new(order: &DepthFirstOrder) -> Self {
        RevPostIter {
            stack: order.post.clone(),
        }
    }
}

impl Iterator for RevPostIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_dag() {
        let mut dag = Digraph::new(13);
        dag.add_edge(2, 3);
        dag.add_edge(0, 6);
        dag.add_edge(0, 1);
        dag.add_edge(2, 0);
        dag.add_edge(11, 12);
        dag.add_edge(9, 12);
        dag.add_edge(9, 11);
        dag.add_edge(9, 10);
        dag.add_edge(3, 5);
        dag.add_edge(8, 7);
        dag.add_edge(5, 4);
        dag.add_edge(0, 5);
        dag.add_edge(6, 4);
        dag.add_edge(6, 9);
        dag.add_edge(7, 6);

        let order = DepthFirstOrder::new(&dag);

        assert_eq!(
            order.pre().collect::<Vec<usize>>(),
            vec![0, 6, 4, 9, 12, 11, 10, 1, 5, 2, 3, 7, 8]
        );
        assert_eq!(
            order.post().collect::<Vec<usize>>(),
            vec![4, 12, 11, 10, 9, 6, 1, 5, 0, 3, 2, 7, 8]
        );
        assert_eq!(
            order.rev_post().collect::<Vec<usize>>(),
            vec![8, 7, 2, 3, 0, 5, 1, 6, 9, 10, 11, 12, 4]
        );
    }
}
