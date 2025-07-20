//! # A edge-weighted digraph of vertices named 0 to (v-1)

use super::directed_edge::DirectedEdge;
pub struct EdgeWeightedDiagraph {
    v: usize,
    e: usize,
    adj: Vec<Vec<DirectedEdge>>,
    in_degree: Vec<usize>,
}

impl EdgeWeightedDiagraph {
    pub fn new(v: usize) -> Self {
        EdgeWeightedDiagraph {
            v,
            e: 0,
            adj: vec![vec![]; v],
            in_degree: vec![0; v],
        }
    }

    /// Returns the number of vertices in this edge-weighted digraph.
    pub fn v(&self) -> usize {
        self.v
    }

    /// Returns the number of edges in this edge-weighted digraph.
    pub fn e(&self) -> usize {
        self.e
    }

    /// Adds the directed edge to this edge-weighted digraph.
    pub fn add_edge(&mut self, e: DirectedEdge) {
        let v = e.from();
        let w = e.to();
        self.adj[v].push(e);
        self.in_degree[w] += 1;
        self.e += 1;
    }

    /// Returns the directed edges incident from vertex v
    pub fn adj(&self, v: usize) -> std::vec::IntoIter<DirectedEdge> {
        self.adj[v].clone().into_iter()
    }

    /// Outdegree of vertex v
    pub fn out_degree(&self, v: usize) -> usize {
        self.adj[v].len()
    }

    /// Indegree of vertex v
    pub fn in_degree(&self, v: usize) -> usize {
        self.in_degree[v]
    }

    /// Returns all directed edges in this edge-weighted digraph.
    pub fn edges(&self) -> std::vec::IntoIter<DirectedEdge> {
        self.adj
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<DirectedEdge>>()
            .into_iter()
    }
}

impl std::fmt::Display for EdgeWeightedDiagraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.v, self.e)?;
        for (i, adj) in self.adj.iter().enumerate() {
            write!(f, "{i}: ")?;
            for edge in adj {
                write!(f, "{edge}; ")?;
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
        let mut g = EdgeWeightedDiagraph::new(8);
        g.add_edge(DirectedEdge::new(4, 5, 0.35));
        g.add_edge(DirectedEdge::new(5, 4, 0.35));
        g.add_edge(DirectedEdge::new(4, 7, 0.37));
        g.add_edge(DirectedEdge::new(5, 7, 0.28));
        g.add_edge(DirectedEdge::new(7, 5, 0.28));
        g.add_edge(DirectedEdge::new(5, 1, 0.32));
        g.add_edge(DirectedEdge::new(0, 4, 0.38));
        g.add_edge(DirectedEdge::new(0, 2, 0.26));
        g.add_edge(DirectedEdge::new(7, 3, 0.39));
        g.add_edge(DirectedEdge::new(1, 3, 0.29));
        g.add_edge(DirectedEdge::new(2, 7, 0.34));
        g.add_edge(DirectedEdge::new(6, 2, 0.40));
        g.add_edge(DirectedEdge::new(3, 6, 0.52));
        g.add_edge(DirectedEdge::new(6, 0, 0.58));
        g.add_edge(DirectedEdge::new(6, 4, 0.93));

        println!("{g}");
    }
}
