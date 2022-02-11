//! # finding shortest paths from a source vertex to every other
//! vertex in the digraph
//!
//! This implementation uses breadth-first search.

use std::collections::VecDeque;

use super::digraph::Digraph;
pub struct BreadthFirstDirectedPaths {
    marked: Vec<bool>,   // is there an s->v path?
    edge_to: Vec<usize>, // last edge on shortest s->v path
    dist_to: Vec<usize>, // length of shortest s->v path
    s: usize,            // source
}

impl BreadthFirstDirectedPaths {
    pub fn new(g: &Digraph, s: usize) -> Self {
        let mut path = BreadthFirstDirectedPaths {
            marked: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            dist_to: vec![usize::MAX; g.v()],
            s,
        };
        path.dfs(g, s);
        path
    }

    fn dfs(&mut self, g: &Digraph, s: usize) {
        self.marked[s] = true;
        self.dist_to[s] = 0;
        let mut q = VecDeque::new();
        q.push_back(s);
        while let Some(v) = q.pop_front() {
            for w in g.adj(v).clone() {
                if !self.marked[w] {
                    self.edge_to[w] = v;
                    self.dist_to[w] = self.dist_to[v] + 1;
                    self.marked[w] = true;
                    q.push_back(w);
                }
            }
        }
    }

    /// Is there a directed path from the source to v
    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    /// Returns the number of edges in a shortest path from the source to v
    pub fn dist_to(&self, v: usize) -> usize {
        self.dist_to[v]
    }

    /// Returns a shortest path from the source to v
    pub fn path_to(&self, v: usize) -> Iter {
        Iter::new(self, v)
    }
}

pub struct Iter {
    stack: Vec<usize>,
}

impl Iter {
    pub fn new(path: &BreadthFirstDirectedPaths, v: usize) -> Self {
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

        let search = BreadthFirstDirectedPaths::new(&digraph, 3);

        assert_eq!(search.has_path_to(0), true);
        assert_eq!(search.dist_to(0), 2);
        assert_eq!(search.path_to(0).collect::<Vec<usize>>(), vec![3, 2, 0]);
    }
}
