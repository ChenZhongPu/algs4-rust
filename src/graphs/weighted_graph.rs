//! # An edge-weighted undirected graph
//!
//! It is implemented using adjacency lists.

use super::edge::Edge;
pub struct EdgeWeightedGraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<Edge>>,
}

impl EdgeWeightedGraph {
    /// Initializes an empty edge-weighted graph with v vertices and 0 edges.
    pub fn new(v: usize) -> Self {
        EdgeWeightedGraph {
            v,
            e: 0,
            adj: vec![vec![]; v],
        }
    }
    /// Adds the undirected edge to this edge-weighted graph.
    pub fn add_edge(&mut self, e: Edge) {
        let v = e.either();
        let w = e.other(v);
        self.adj[v].push(e.clone());
        self.adj[w].push(e);
        self.e += 1;
    }

    /// Returns the number of vertices in this edge-weighted graph.
    pub fn v(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in this edge-weighted graph.
    pub fn e(&self) -> usize {
        self.e
    }

    /// Returns the edges incident on vertex v
    pub fn adj(&self, v: usize) -> std::vec::IntoIter<Edge> {
        self.adj[v].clone().into_iter()
    }

    /// Returns all edges in this graph.
    pub fn edges(&self) -> std::vec::IntoIter<Edge> {
        let mut list = Vec::new();
        for v in 0..self.v {
            let mut self_loops = 0;
            for e in self.adj(v) {
                match e.other(v).cmp(&v) {
                    std::cmp::Ordering::Less => {}
                    std::cmp::Ordering::Equal => {
                        // add only one copy of each self-loop
                        if self_loops % 2 == 0 {
                            list.push(e);
                        }
                        self_loops += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        list.push(e);
                    }
                }
            }
        }
        list.into_iter()
    }
}

impl std::fmt::Display for EdgeWeightedGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.v, self.e)?;
        for v in 0..self.v {
            write!(f, "{v}: ")?;
            for edge in self.adj(v) {
                write!(f, "{edge}  ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tiny_ewg() {
        let mut g = EdgeWeightedGraph::new(8);
        g.add_edge(Edge::new(4, 5, 0.35));
        g.add_edge(Edge::new(4, 7, 0.37));
        g.add_edge(Edge::new(5, 7, 0.28));
        g.add_edge(Edge::new(0, 7, 0.16));
        g.add_edge(Edge::new(1, 5, 0.32));
        g.add_edge(Edge::new(0, 4, 0.38));
        g.add_edge(Edge::new(2, 3, 0.17));
        g.add_edge(Edge::new(1, 7, 0.19));
        g.add_edge(Edge::new(0, 2, 0.26));
        g.add_edge(Edge::new(1, 2, 0.36));
        g.add_edge(Edge::new(1, 3, 0.29));
        g.add_edge(Edge::new(2, 7, 0.34));
        g.add_edge(Edge::new(6, 2, 0.40));
        g.add_edge(Edge::new(3, 6, 0.52));
        g.add_edge(Edge::new(6, 0, 0.58));
        g.add_edge(Edge::new(6, 4, 0.93));
        println!("{g}");

        assert_eq!(g.e(), 16);
    }
}
