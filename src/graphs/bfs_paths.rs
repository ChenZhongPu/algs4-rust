//! # Finding shortest paths from a source to every other vertex in an undirected graph
//!
//! This implementation uses bread-first search.
//! Note if the shortest paths are more than one, the result relies on `adj` order.

use std::collections::VecDeque;

use super::graph::Graph;

pub struct BreadFirstPaths {
    marked: Vec<bool>,   // is a shortest path to this vertex known?
    edge_to: Vec<usize>, // last vertex on known path to this vertex
    source: usize,
}

impl BreadFirstPaths {
    pub fn new(g: &Graph, source: usize) -> BreadFirstPaths {
        let mut paths = BreadFirstPaths {
            marked: vec![false; g.v()],
            edge_to: vec![0; g.v()],
            source,
        };
        paths.bfs(g, source);
        paths
    }

    fn bfs(&mut self, g: &Graph, s: usize) {
        let mut queue = VecDeque::new();
        self.marked[s] = true;
        queue.push_back(s);
        while let Some(v) = queue.pop_front() {
            for w in g.adj(v).clone() {
                if !self.marked[w] {
                    // save last edge on a shortest path
                    self.edge_to[w] = v;
                    self.marked[w] = true;
                    queue.push_back(w);
                }
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Vec<usize> {
        if !self.has_path_to(v) {
            return vec![];
        }
        let mut paths = Vec::new();
        let mut x = v;
        while x != self.source {
            paths.insert(0, x);
            x = self.edge_to[x];
        }
        paths.insert(0, self.source);
        paths
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

        assert_eq!(paths.path_to(2), vec![0, 2]);
        assert_eq!(paths.path_to(3), vec![0, 5, 3]);
        assert_eq!(paths.path_to(4), vec![0, 2, 4]);
    }
}
