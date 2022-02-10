//! # determining whether an undirected graph is *bipartite* or whether it has an *odd-length cycle*.
//!
//! A graph is bipartite if and only if it has no odd-length cycle.

use super::graph::Graph;
pub struct Bipartite {
    is_bipartite: bool,
    color: Vec<bool>,  // color[v] gives vertices on one side of bipartition
    marked: Vec<bool>, // marked[v] = true iff v has been visited in DFS
}

impl Bipartite {
    pub fn new(g: &Graph) -> Bipartite {
        let mut bipartite = Bipartite {
            is_bipartite: true,
            color: vec![false; g.v()],
            marked: vec![false; g.v()],
        };
        for s in 0..g.v() {
            if !bipartite.marked[s] {
                bipartite.dfs(g, s);
            }
        }
        bipartite
    }

    fn dfs(&mut self, g: &Graph, v: usize) {
        self.marked[v] = true;

        for w in g.adj(v).clone() {
            // found uncolored/unvisited vertex
            if !self.marked[w] {
                self.color[w] = !self.color[v];
                self.dfs(g, w);
            } else if self.color[w] == self.color[v] {
                // if v-w create an odd-length cycle
                self.is_bipartite = false;
                return;
            }
        }
    }

    pub fn is_bipartite(&self) -> bool {
        self.is_bipartite
    }
}

#[cfg(test)]
mod test {
    use crate::graphs::graph_generator;

    use super::*;

    #[test]
    fn one_color() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 5);
        graph.add_edge(2, 4);
        graph.add_edge(2, 3);
        graph.add_edge(1, 2);
        graph.add_edge(0, 1);
        graph.add_edge(3, 4);
        graph.add_edge(3, 5);
        graph.add_edge(0, 2);

        let bipartite = Bipartite::new(&graph);
        assert_eq!(bipartite.is_bipartite(), false);
    }

    #[test]
    fn two_color() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(3, 5);
        graph.add_edge(2, 4);

        let bipartite = Bipartite::new(&graph);
        assert_eq!(bipartite.is_bipartite(), true);
    }

    #[test]
    fn generate_bipartite() {
        let graph = graph_generator::bipartite(5, 8, 18);

        let bipartite = Bipartite::new(&graph);
        assert_eq!(bipartite.is_bipartite(), true);
    }
}
