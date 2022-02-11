//! # Finding shortest paths from a source to every other vertex in an undirected graph
//!
//! This implementation uses bread-first search.
//! Note if the shortest paths are more than one, the result relies on `adj` order.

use std::{collections::VecDeque, vec};

use super::graph::Graph;

pub struct BreadFirstPaths {
    marked: Vec<bool>,   // is a shortest path to this vertex known?
    edge_to: Vec<usize>, // last vertex on known path to this vertex
    dist_to: Vec<usize>,
    source: usize,
}

impl BreadFirstPaths {
    pub fn new(g: &Graph, source: usize) -> BreadFirstPaths {
        let mut paths = BreadFirstPaths {
            marked: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            dist_to: vec![usize::MAX; g.v()],
            source,
        };
        paths.bfs(g, source);
        paths
    }

    fn bfs(&mut self, g: &Graph, s: usize) {
        let mut queue = VecDeque::new();
        self.marked[s] = true;
        self.dist_to[s] = 0;
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            for w in g.adj(v).clone() {
                if !self.marked[w] {
                    // save last edge on a shortest path
                    self.edge_to[w] = v;
                    self.dist_to[w] = self.dist_to[v] + 1;
                    self.marked[w] = true;
                    queue.push_back(w);
                }
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn dist_to(&self, v: usize) -> usize {
        self.dist_to[v]
    }

    pub fn path_to(&self, v: usize) -> Iter {
        Iter::new(self, v)
    }
}

pub struct Iter {
    stack: Vec<usize>,
}

impl Iter {
    pub fn new(path: &BreadFirstPaths, v: usize) -> Self {
        let mut stack = Vec::new();
        if path.has_path_to(v) {
            let mut x = v;
            while x != path.source {
                stack.push(x);
                x = path.edge_to[x];
            }
            stack.push(path.source);
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
    fn bfs_paths() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 5);
        graph.add_edge(2, 4);
        graph.add_edge(2, 3);
        graph.add_edge(1, 2);
        graph.add_edge(0, 1);
        graph.add_edge(3, 4);
        graph.add_edge(3, 5);
        graph.add_edge(0, 2);

        // 0: 5 2 1
        // 1: 2 0
        // 2: 4 3 1 0
        // 3: 2 4 5
        // 4: 2 3
        // 5: 0 3

        let paths = BreadFirstPaths::new(&graph, 0);

        assert_eq!(paths.dist_to(2), 1);
        assert_eq!(paths.path_to(2).collect::<Vec<usize>>(), vec![0, 2]);
        assert_eq!(paths.dist_to(3), 2);
        assert_eq!(paths.path_to(3).collect::<Vec<usize>>(), vec![0, 5, 3]);
        assert_eq!(paths.dist_to(4), 2);
        assert_eq!(paths.path_to(4).collect::<Vec<usize>>(), vec![0, 2, 4]);
    }
}
