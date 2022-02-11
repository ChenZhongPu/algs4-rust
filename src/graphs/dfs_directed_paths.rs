//! # Find directed paths from a source vertex to every other vertex in the digraph.
//!
//! This implementation uses depth-first search.

use super::digraph::Digraph;
pub struct DepthFirstDirectedPaths {
    marked: Vec<bool>,   // marked[v] = true iff v is reachable from s
    edge_to: Vec<usize>, // edge_to[v] = last edge on path from s to v
    s: usize,            // source
}

impl DepthFirstDirectedPaths {
    pub fn new(g: &Digraph, s: usize) -> DepthFirstDirectedPaths {
        let mut path = DepthFirstDirectedPaths {
            marked: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            s,
        };
        path.dfs(g, s);
        path
    }

    fn dfs(&mut self, g: &Digraph, v: usize) {
        self.marked[v] = true;
        for w in g.adj(v).clone() {
            if !self.marked[w] {
                self.edge_to[w] = v;
                self.dfs(g, w);
            }
        }
    }

    /// Is there a directed path from the source to v?
    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    /// Returns a directed path.
    /// Note that the path relies on the order of adj
    /// if multiple paths exist.
    pub fn path_to(&self, v: usize) -> Iter {
        Iter::new(self, v)
    }
}

pub struct Iter {
    stack: Vec<usize>,
}

impl Iter {
    pub fn new(path: &DepthFirstDirectedPaths, v: usize) -> Self {
        let mut stack = Vec::new();
        if path.has_path_to(v) {
            let mut x = v;
            while x != path.s {
                stack.push(x);
                x = path.edge_to[x];
            }
            stack.push(path.s);
        }
        Iter { stack }
    }
}

impl Iterator for Iter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
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

        let search = DepthFirstDirectedPaths::new(&digraph, 3);
        assert_eq!(search.has_path_to(0), true);
        assert_eq!(search.path_to(0).collect::<Vec<usize>>(), vec![3, 2, 0]);

        assert_eq!(search.path_to(1).collect::<Vec<usize>>(), vec![3, 2, 0, 1]);

        assert_eq!(search.has_path_to(6), false);
        assert_eq!(search.path_to(6).collect::<Vec<usize>>(), vec![]);
    }
}
