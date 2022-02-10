//! # Detect a cycle in an undirected graph
//!
//! Assume no self-loops and parallel edges.
//! An undirected graph has a cycle if and only if a depth-first search finds an edge that points to an already-visited vertex (a back edge).

use super::graph::Graph;
pub struct Cycle {
    marked: Vec<bool>,
    has_cycle: bool,
}

impl Cycle {
    pub fn new(g: &Graph) -> Cycle {
        let mut c = Cycle {
            marked: vec![false; g.v()],
            has_cycle: false,
        };
        for s in 0..g.v() {
            if !c.marked[s] {
                c.dfs(g, s, s);
            }
        }
        c
    }

    fn dfs(&mut self, g: &Graph, v: usize, parent: usize) {
        self.marked[v] = true;
        for w in g.adj(v).clone() {
            if !self.marked[w] {
                self.dfs(g, w, v);
            } else if w != parent {
                self.has_cycle = true;
                return;
            }
        }
    }

    pub fn has_cycle(&self) -> bool {
        self.has_cycle
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn detect_cycle() {
        let mut graph = Graph::new(13);
        graph.add_edge(0, 5);
        graph.add_edge(4, 3);
        graph.add_edge(0, 1);
        graph.add_edge(9, 12);
        graph.add_edge(6, 4);
        graph.add_edge(5, 4);
        graph.add_edge(0, 2);
        graph.add_edge(11, 12);
        graph.add_edge(9, 10);
        graph.add_edge(0, 6);
        graph.add_edge(7, 8);
        graph.add_edge(9, 11);
        graph.add_edge(5, 3);

        let cycle = Cycle::new(&graph);
        assert_eq!(cycle.has_cycle(), true);
    }

    #[test]
    fn acyclic() {
        let mut graph = Graph::new(6);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(3, 5);
        graph.add_edge(2, 4);

        let cycle = Cycle::new(&graph);
        assert_eq!(cycle.has_cycle(), false);
    }
}
