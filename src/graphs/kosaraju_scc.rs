//! # Determining the strong components in a digraph.
//!
//! This implementation uses the Kosaraju-Sharir algorithm.
//! The time complexity is O(V + E).

use super::{dfs_order::DepthFirstOrder, digraph::Digraph};
pub struct KosarajuSCC {
    marked: Vec<bool>, // reached vertices
    id: Vec<usize>,    // component identifiers
    count: usize,      // number of strong components
}

impl KosarajuSCC {
    pub fn new(g: &Digraph) -> Self {
        let mut scc = KosarajuSCC {
            marked: vec![false; g.v()],
            id: vec![0; g.v()],
            count: 0,
        };
        let order = DepthFirstOrder::new(&g.reverse());
        for s in order.rev_post() {
            if !scc.marked[s] {
                scc.dfs(g, s);
                scc.count += 1;
            }
        }
        scc
    }

    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        self.id[v] = self.count;
        for w in g.adj(v).clone() {
            if !self.marked[w] {
                self.dfs(g, w);
            }
        }
    }

    /// Are v and w strongly connected?
    pub fn strongly_connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
    }

    /// Returns the identifier of the strong component of v
    pub fn id(&self, v: usize) -> usize {
        self.id[v]
    }

    /// Returns the number of strong components
    pub fn count(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_dg() {
        let mut digraph = Digraph::new(13);
        digraph.add_edge(4, 2);
        digraph.add_edge(2, 3);
        digraph.add_edge(3, 2);
        digraph.add_edge(6, 0);
        digraph.add_edge(0, 1);
        digraph.add_edge(2, 0);
        digraph.add_edge(11, 12);
        digraph.add_edge(12, 9);
        digraph.add_edge(9, 10);
        digraph.add_edge(9, 11);
        digraph.add_edge(8, 9);
        digraph.add_edge(10, 12);
        digraph.add_edge(11, 4);
        digraph.add_edge(4, 3);
        digraph.add_edge(3, 5);
        digraph.add_edge(7, 8);
        digraph.add_edge(8, 7);
        digraph.add_edge(5, 4);
        digraph.add_edge(0, 5);
        digraph.add_edge(6, 4);
        digraph.add_edge(6, 9);
        digraph.add_edge(7, 6);

        let scc = KosarajuSCC::new(&digraph);

        assert_eq!(scc.count(), 5);

        assert_eq!(scc.strongly_connected(0, 2), true);
        assert_eq!(scc.strongly_connected(2, 3), true);
        assert_eq!(scc.strongly_connected(3, 4), true);
        assert_eq!(scc.strongly_connected(4, 5), true);

        assert_eq!(scc.strongly_connected(0, 1), false);

        assert_eq!(scc.strongly_connected(7, 8), true);
        assert_eq!(scc.strongly_connected(0, 7), false);
    }
}
