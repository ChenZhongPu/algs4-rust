//! # A naive solution to strongly connected components in a diagraph.
//!
//! Time complexity: $O(n^2)$.

use std::collections::{HashSet, VecDeque};

use super::digraph::Digraph;
pub struct NaiveSCC {
    marked: Vec<bool>,
    id: Vec<usize>, // component identifier for v (between 0 and count()-1)
    count: usize,   // number of strong components
}

impl NaiveSCC {
    pub fn new(g: &Digraph) -> Self {
        let mut scc = NaiveSCC {
            marked: vec![false; g.v()],
            id: vec![0; g.v()],
            count: 0,
        };
        for v in 0..g.v() {
            if !scc.marked[v] {
                scc.component(g, v);
            }
        }
        scc
    }

    fn component(&mut self, g: &Digraph, s: usize) {
        self.marked[s] = true;
        self.id[s] = self.count;
        // find the set of vertices reachable from s
        let reachable = self.bfs(g, s);
        let rev_g = g.reverse();
        // find the set of vertices reaching to s
        let reaching = self.bfs(&rev_g, s);
        // the intersection belongs to the same strong component with s
        for v in reachable.intersection(&reaching) {
            self.marked[*v] = true;
            self.id[*v] = self.count;
        }

        self.count += 1;
    }

    fn bfs(&mut self, g: &Digraph, s: usize) -> HashSet<usize> {
        let mut result = HashSet::new();
        let mut queue = VecDeque::new();
        let mut visited = vec![false; g.v()];
        visited[s] = true;
        queue.push_back(s);

        while let Some(v) = queue.pop_front() {
            for w in g.adj(v).clone() {
                if !visited[w] {
                    result.insert(w);
                    queue.push_back(w);
                    visited[w] = true;
                }
            }
        }

        result
    }

    /// Returns the number of strong components
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns the identifier of the strong component of v
    pub fn id(&self, v: usize) -> usize {
        self.id[v]
    }

    // Are v and w strongly connected?
    pub fn strongly_connected(&self, v: usize, w: usize) -> bool {
        self.id[v] == self.id[w]
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

        let scc = NaiveSCC::new(&digraph);

        assert_eq!(scc.count(), 5);

        assert!(scc.strongly_connected(0, 2));
        assert!(scc.strongly_connected(2, 3));
        assert!(scc.strongly_connected(3, 4));
        assert!(scc.strongly_connected(4, 5));

        assert!(!scc.strongly_connected(0, 1));

        assert!(scc.strongly_connected(7, 8));
        assert!(!scc.strongly_connected(0, 7));
    }
}
