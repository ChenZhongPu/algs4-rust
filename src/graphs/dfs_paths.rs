//! # Finding paths from a source vertex to every other vertex in an undirected graph
//!
//! This implementation uses depth-first search.
//! Note that the paths rely on `adj` orders.
use super::graph::Graph;

pub struct DepthFirstPaths {
    marked: Vec<bool>,
    edge_to: Vec<i32>, // last vertex on known path to this vertex
    source: usize,
}

impl DepthFirstPaths {
    pub fn new(g: &Graph, s: usize) -> DepthFirstPaths {
        DepthFirstPaths {
            marked: vec![false; g.v()],
            edge_to: vec![-1; g.v()],
            source: s,
        }
    }

    pub fn dfs(&mut self, g: &Graph) {
        self._dfs(g, self.source);
    }

    fn _dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;

        for w in g.adj(v).clone() {
            if !self.marked[w] {
                self.edge_to[w] = v as i32;
                self._dfs(g, w);
            }
        }
    }

    pub fn has_path_to(&self, v: usize) -> bool {
        self.marked[v]
    }

    pub fn path_to(&self, v: usize) -> Vec<usize> {
        if !self.has_path_to(v) {
            return Vec::new();
        }
        let mut paths = Vec::new();
        let mut x = v;
        while x != self.source {
            paths.insert(0, x);
            x = self.edge_to[x] as usize;
        }
        paths.insert(0, self.source);
        paths
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dfs_path() {
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

        let mut df_path = DepthFirstPaths::new(&graph, 0);
        df_path.dfs(&graph);

        assert_eq!(df_path.path_to(5), vec![0, 5]);

        assert_eq!(df_path.path_to(2), vec![0, 5, 3, 2]);
    }
}
